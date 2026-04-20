use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, ensure, Context, Result};
use midly::{
    num::{u15, u24, u28, u4, u7},
    Format, Header, MetaMessage, MidiMessage, Smf, Timing, TrackEvent, TrackEventKind,
};
use nalgebra::{DMatrix, DVector};
use rand::{rngs::StdRng, Rng, SeedableRng};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::audio_engine::AudioEngine;
use crate::midi_model::{MidiModel, MidiNote};
use crate::state_space::StateSpaceSystem;

/// Name of the built-in demo preset.
pub const DEMO_PRESET_NAME: &str = "demo";

/// Serializable representation of a dense matrix (row-major).
///
/// Used to persist `nalgebra::DMatrix<f64>` values inside preset JSON files.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatrixSpec {
    /// Number of rows.
    pub rows: usize,
    /// Number of columns.
    pub cols: usize,
    /// Row-major element data. Length must equal `rows * cols`.
    pub data: Vec<f64>,
}

impl MatrixSpec {
    /// Creates a `MatrixSpec` from an existing `DMatrix`.
    pub fn from_matrix(matrix: &DMatrix<f64>) -> Self {
        Self {
            rows: matrix.nrows(),
            cols: matrix.ncols(),
            data: matrix.iter().copied().collect(),
        }
    }

    /// Reconstructs a `DMatrix` from this spec. Returns an error if `data.len() != rows * cols`.
    pub fn to_matrix(&self) -> Result<DMatrix<f64>> {
        ensure!(
            self.data.len() == self.rows * self.cols,
            "matrix data length {} does not match {}x{}",
            self.data.len(),
            self.rows,
            self.cols
        );

        Ok(DMatrix::from_row_slice(self.rows, self.cols, &self.data))
    }
}

/// Serializable form of a [`StateSpaceSystem`] (matrices A, B, C, D plus optional timestep).
///
/// Stored inside [`RenderPreset`] so presets are fully self-contained JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSpacePresetSpec {
    /// State transition matrix.
    pub a: MatrixSpec,
    /// Input matrix.
    pub b: MatrixSpec,
    /// Output matrix.
    pub c: MatrixSpec,
    /// Feed-through matrix.
    pub d: MatrixSpec,
    /// Discrete timestep. `None` means continuous (Euler integration is used).
    pub dt: Option<f64>,
}

impl StateSpacePresetSpec {
    /// Snapshots a live [`StateSpaceSystem`] into a serializable spec.
    pub fn from_system(system: &StateSpaceSystem) -> Self {
        Self {
            a: MatrixSpec::from_matrix(&system.a),
            b: MatrixSpec::from_matrix(&system.b),
            c: MatrixSpec::from_matrix(&system.c),
            d: MatrixSpec::from_matrix(&system.d),
            dt: system.dt,
        }
    }

    /// Rebuilds a [`StateSpaceSystem`] from this spec. Fails if matrix dimensions are inconsistent.
    pub fn to_system(&self) -> Result<StateSpaceSystem> {
        StateSpaceSystem::new(
            self.a.to_matrix()?,
            self.b.to_matrix()?,
            self.c.to_matrix()?,
            self.d.to_matrix()?,
            self.dt,
        )
        .map_err(|err| anyhow!(err.to_string()))
    }
}

/// Parameters that control how the state-space system is numerically simulated.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationConfig {
    /// Total simulation length in seconds.
    pub duration_seconds: f64,
    /// Frames per second used when sampling the trajectory.
    pub trajectory_sample_rate: u32,
    /// Initial state vector. Length must match the system's state dimension.
    pub initial_state: Vec<f64>,
    /// Constant input vector applied at every step. Length must match the system's input dimension.
    pub input: Vec<f64>,
}

/// Controls how a trajectory is mapped to MIDI notes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MidiMappingConfig {
    /// Tempo in beats per minute.
    pub tempo_bpm: u16,
    /// MIDI ticks per quarter note (resolution of the MIDI file).
    pub ticks_per_beat: u16,
    /// MIDI channel (0-15).
    pub channel: u8,
    /// Base velocity before energy-based adjustment.
    pub default_velocity: u8,
    /// Lowest allowed MIDI note number.
    pub low_note: u8,
    /// Highest allowed MIDI note number.
    pub high_note: u8,
    /// Duration of each note step in beats.
    pub step_beats: f64,
    /// Root note for scale quantization (MIDI note number).
    pub root_note: u8,
    /// Scale intervals relative to `root_note` (e.g. `[0, 2, 4, 7, 9]` for pentatonic).
    pub scale: Vec<u8>,
    /// Maximum random pitch deviation in semitones, seeded by the generation seed.
    pub seed_variation_semitones: u8,
}

/// Audio rendering parameters for WAV output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioRenderConfig {
    /// Sample rate in Hz (e.g. 44100).
    pub sample_rate: u32,
    /// Peak limiter threshold (0.0 .. 1.0).
    pub peak_limit: f32,
    /// Note attack envelope duration in seconds.
    pub attack_seconds: f64,
    /// Note release envelope duration in seconds.
    pub release_seconds: f64,
}

/// A complete, self-contained preset that fully specifies a deterministic composition.
///
/// Combines the state-space system definition with simulation, MIDI mapping, and audio
/// rendering parameters. Presets are stored as JSON files under the `presets/` directory.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderPreset {
    /// Unique preset identifier (also used as the filename stem).
    pub name: String,
    /// Human-readable description of the preset's character.
    pub description: String,
    /// State-space system matrices.
    pub system: StateSpacePresetSpec,
    /// Simulation parameters (duration, sample rate, initial conditions).
    pub simulation: SimulationConfig,
    /// MIDI mapping configuration (scale, tempo, note range).
    pub midi: MidiMappingConfig,
    /// Audio rendering settings (sample rate, envelope, limiter).
    pub audio: AudioRenderConfig,
}

impl RenderPreset {
    /// Creates a preset with sensible defaults derived from the given system's dimensions.
    ///
    /// Defaults: 8-second duration, 120 BPM, pentatonic scale, 44.1 kHz audio.
    pub fn from_system(name: String, description: String, system: &StateSpaceSystem) -> Self {
        let state_dim = system.a.nrows();
        let input_dim = system.b.ncols();

        let mut initial_state = vec![0.0; state_dim];
        if let Some(first) = initial_state.first_mut() {
            *first = 1.0;
        }

        Self {
            name,
            description,
            system: StateSpacePresetSpec::from_system(system),
            simulation: SimulationConfig {
                duration_seconds: 8.0,
                trajectory_sample_rate: 256,
                initial_state,
                input: vec![0.0; input_dim],
            },
            midi: MidiMappingConfig {
                tempo_bpm: 120,
                ticks_per_beat: 480,
                channel: 0,
                default_velocity: 96,
                low_note: 48,
                high_note: 84,
                step_beats: 0.5,
                root_note: 60,
                scale: vec![0, 2, 4, 7, 9],
                seed_variation_semitones: 4,
            },
            audio: AudioRenderConfig {
                sample_rate: 44_100,
                peak_limit: 0.85,
                attack_seconds: 0.01,
                release_seconds: 0.05,
            },
        }
    }
}

/// Lightweight entry returned by [`list_presets`] for discovery without loading full preset data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresetSummary {
    /// Preset name (matches the JSON filename stem or `"demo"` for the built-in).
    pub name: String,
    /// Origin: `"builtin"` for the demo preset, or the file path for user presets.
    pub source: String,
}

/// A single sampled point from a state-space simulation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrajectoryFrame {
    /// Wall-clock time of this frame relative to simulation start.
    pub time_seconds: f64,
    /// Full state vector at this frame.
    pub state: Vec<f64>,
    /// Output vector (C*x + D*u) at this frame.
    pub output: Vec<f64>,
}

/// Aggregate statistics of a simulated trajectory, useful for quick inspection.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct TrajectorySummary {
    /// Total number of frames in the trajectory.
    pub frame_count: usize,
    /// Duration covered by the trajectory in seconds.
    pub duration_seconds: f64,
    /// Minimum primary output value across all frames.
    pub min_output: f64,
    /// Maximum primary output value across all frames.
    pub max_output: f64,
    /// Mean of absolute primary output values.
    pub mean_abs_output: f64,
    /// Peak absolute primary output value.
    pub peak_abs_output: f64,
    /// First few primary output values (up to 8) for a quick visual preview.
    pub preview: Vec<f64>,
}

/// The full output of [`generate_composition`]: trajectory, MIDI, and rendered audio.
///
/// All fields are deterministic for a given `(preset, seed)` pair.
#[derive(Debug, Clone)]
pub struct GeneratedComposition {
    /// The preset used for this generation.
    pub preset: RenderPreset,
    /// RNG seed that was applied. Same seed + same preset = identical output.
    pub seed: u64,
    /// Raw simulation frames.
    pub trajectory: Vec<TrajectoryFrame>,
    /// Aggregate statistics of the trajectory.
    pub trajectory_summary: TrajectorySummary,
    /// MIDI note sequence derived from the trajectory.
    pub midi_model: MidiModel,
    /// Rendered audio samples (mono, f32, at `preset.audio.sample_rate`).
    pub audio_samples: Vec<f32>,
}

/// Metadata returned after writing a MIDI file to disk.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct MidiArtifactSummary {
    /// Path where the `.mid` file was written.
    pub path: PathBuf,
    /// Number of MIDI notes in the file.
    pub note_count: usize,
    /// Total duration of the piece in beats.
    pub duration_beats: f64,
    /// Tempo embedded in the MIDI file.
    pub tempo_bpm: u16,
    /// File size in bytes.
    pub bytes_written: u64,
    /// SHA-256 hex digest of the written file for integrity verification.
    pub artifact_hash: String,
}

/// Metadata returned after writing a WAV file to disk.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct WavArtifactSummary {
    /// Path where the `.wav` file was written.
    pub path: PathBuf,
    /// Total number of audio samples.
    pub sample_count: usize,
    /// Duration of the audio in seconds.
    pub duration_seconds: f64,
    /// Peak absolute sample amplitude.
    pub peak_amplitude: f32,
    /// Sample rate in Hz.
    pub sample_rate: u32,
    /// SHA-256 hex digest of the written file for integrity verification.
    pub artifact_hash: String,
}

/// Combined summary returned by [`export_demo_artifacts`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemoArtifactSummary {
    /// Name of the preset that was used.
    pub preset: String,
    /// RNG seed that was applied.
    pub seed: u64,
    /// Trajectory statistics for the generated composition.
    pub trajectory: TrajectorySummary,
    /// MIDI artifact metadata, present if a MIDI output path was provided.
    pub midi: Option<MidiArtifactSummary>,
    /// WAV artifact metadata, present if a WAV output path was provided.
    pub wav: Option<WavArtifactSummary>,
}

/// Returns the default preset directory path (`presets/`).
pub fn default_preset_dir() -> PathBuf {
    PathBuf::from("presets")
}

/// Returns the built-in demo preset: a damped oscillator producing a pentatonic melody.
pub fn demo_preset() -> RenderPreset {
    let system = StateSpaceSystem::new(
        DMatrix::from_row_slice(2, 2, &[0.0, 1.0, -1.0, -0.15]),
        DMatrix::zeros(2, 0),
        DMatrix::from_row_slice(1, 2, &[1.0, 0.0]),
        DMatrix::zeros(1, 0),
        None,
    )
    .expect("demo preset system should be valid");

    let mut preset = RenderPreset::from_system(
        DEMO_PRESET_NAME.to_string(),
        "Built-in oscillator demo preset".to_string(),
        &system,
    );
    preset.simulation.duration_seconds = 8.0;
    preset.simulation.trajectory_sample_rate = 256;
    preset.audio.sample_rate = 44_100;
    preset
}

/// Lists all available presets: the built-in demo plus any `.json` files in `preset_dir`.
///
/// Results are sorted alphabetically by name. The directory need not exist (only the
/// built-in demo will be returned in that case).
pub fn list_presets(preset_dir: &Path) -> Result<Vec<PresetSummary>> {
    let mut presets = vec![PresetSummary {
        name: DEMO_PRESET_NAME.to_string(),
        source: "builtin".to_string(),
    }];

    if preset_dir.exists() {
        for entry in fs::read_dir(preset_dir)
            .with_context(|| format!("failed to read preset directory {}", preset_dir.display()))?
        {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
                continue;
            }

            if let Some(stem) = path.file_stem().and_then(|stem| stem.to_str()) {
                if stem != DEMO_PRESET_NAME {
                    presets.push(PresetSummary {
                        name: stem.to_string(),
                        source: path.display().to_string(),
                    });
                }
            }
        }
    }

    presets.sort_by(|left, right| left.name.cmp(&right.name));
    Ok(presets)
}

/// Loads a preset by name. Returns the built-in demo for `"demo"`, otherwise reads
/// `<preset_dir>/<name>.json`.
pub fn load_preset(name: &str, preset_dir: &Path) -> Result<RenderPreset> {
    if name == DEMO_PRESET_NAME {
        return Ok(demo_preset());
    }

    let path = preset_dir.join(format!("{name}.json"));
    let raw = fs::read_to_string(&path)
        .with_context(|| format!("failed to read preset file {}", path.display()))?;
    serde_json::from_str(&raw)
        .with_context(|| format!("failed to parse preset file {}", path.display()))
}

/// Writes a preset as pretty-printed JSON to `<preset_dir>/<preset.name>.json`.
///
/// Creates the directory if it does not exist. Returns the path of the written file.
pub fn save_preset(preset: &RenderPreset, preset_dir: &Path) -> Result<PathBuf> {
    fs::create_dir_all(preset_dir)
        .with_context(|| format!("failed to create preset directory {}", preset_dir.display()))?;
    let path = preset_dir.join(format!("{}.json", preset.name));
    let raw = serde_json::to_string_pretty(preset)?;
    fs::write(&path, raw).with_context(|| format!("failed to write {}", path.display()))?;
    Ok(path)
}

/// Runs a numerical simulation of the state-space system and returns sampled frames.
///
/// Uses Euler integration for continuous systems (`dt = None`) or direct step for
/// discrete systems. The number of frames equals `duration_seconds * trajectory_sample_rate`.
pub fn simulate_trajectory(
    system: &StateSpaceSystem,
    simulation: &SimulationConfig,
) -> Result<Vec<TrajectoryFrame>> {
    let frame_count = (simulation.duration_seconds.max(0.0)
        * f64::from(simulation.trajectory_sample_rate))
    .round() as usize;
    let frame_count = frame_count.max(1);
    let integration_dt = system
        .dt
        .unwrap_or_else(|| 1.0 / f64::from(simulation.trajectory_sample_rate.max(1)));

    ensure!(
        simulation.initial_state.len() == system.a.nrows(),
        "initial state length {} does not match system state dimension {}",
        simulation.initial_state.len(),
        system.a.nrows()
    );
    ensure!(
        simulation.input.len() == system.b.ncols(),
        "input length {} does not match system input dimension {}",
        simulation.input.len(),
        system.b.ncols()
    );

    let mut state = DVector::from_vec(simulation.initial_state.clone());
    let input = DVector::from_vec(simulation.input.clone());
    let mut frames = Vec::with_capacity(frame_count);

    for frame_index in 0..frame_count {
        let next_state = if system.dt.is_some() {
            system
                .predict(&state, &input)
                .map_err(|err| anyhow!(err.to_string()))?
        } else {
            let derivative = system
                .predict(&state, &input)
                .map_err(|err| anyhow!(err.to_string()))?;
            &state + derivative * integration_dt
        };

        let output = system
            .output(&next_state, &input)
            .map_err(|err| anyhow!(err.to_string()))?;
        let time_seconds = frame_index as f64 / f64::from(simulation.trajectory_sample_rate.max(1));

        frames.push(TrajectoryFrame {
            time_seconds,
            state: next_state.as_slice().to_vec(),
            output: output.as_slice().to_vec(),
        });
        state = next_state;
    }

    Ok(frames)
}

/// Computes aggregate statistics (min, max, mean, peak, preview) over a trajectory's
/// primary output channel.
pub fn summarize_trajectory(trajectory: &[TrajectoryFrame]) -> TrajectorySummary {
    let outputs: Vec<f64> = trajectory.iter().map(primary_output_value).collect();

    let min_output = outputs.iter().copied().fold(f64::INFINITY, f64::min);
    let max_output = outputs.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let mean_abs_output = if outputs.is_empty() {
        0.0
    } else {
        outputs.iter().map(|value| value.abs()).sum::<f64>() / outputs.len() as f64
    };
    let peak_abs_output = outputs.iter().map(|value| value.abs()).fold(0.0, f64::max);
    let duration_seconds = trajectory
        .last()
        .map(|frame| frame.time_seconds)
        .unwrap_or_default();

    TrajectorySummary {
        frame_count: trajectory.len(),
        duration_seconds,
        min_output: if min_output.is_finite() {
            min_output
        } else {
            0.0
        },
        max_output: if max_output.is_finite() {
            max_output
        } else {
            0.0
        },
        mean_abs_output,
        peak_abs_output,
        preview: outputs.into_iter().take(8).collect(),
    }
}

/// End-to-end deterministic composition: simulate, map to MIDI, render audio.
///
/// Given a preset and a seed, produces a [`GeneratedComposition`] containing the trajectory,
/// MIDI model, and audio samples. The same `(preset, seed)` always yields identical output.
pub fn generate_composition(preset: RenderPreset, seed: u64) -> Result<GeneratedComposition> {
    let system = preset.system.to_system()?;
    let trajectory = simulate_trajectory(&system, &preset.simulation)?;
    let trajectory_summary = summarize_trajectory(&trajectory);
    let midi_model = map_trajectory_to_midi(&preset, &trajectory, seed)?;

    let audio_engine = AudioEngine::new();
    let audio_samples = audio_engine.render_midi_model(
        &midi_model,
        preset.midi.tempo_bpm,
        preset.audio.sample_rate,
        preset.audio.attack_seconds,
        preset.audio.release_seconds,
        preset.audio.peak_limit,
    );

    Ok(GeneratedComposition {
        preset,
        seed,
        trajectory,
        trajectory_summary,
        midi_model,
        audio_samples,
    })
}

/// Writes the MIDI data from a [`GeneratedComposition`] to a `.mid` file.
///
/// Returns a [`MidiArtifactSummary`] with note count, duration, and file hash.
pub fn export_generated_midi(
    composition: &GeneratedComposition,
    output_path: &Path,
) -> Result<MidiArtifactSummary> {
    write_midi_file(
        &composition.midi_model,
        &composition.preset.midi,
        output_path,
    )
}

/// Writes the audio samples from a [`GeneratedComposition`] to a `.wav` file.
///
/// Returns a [`WavArtifactSummary`] with sample count, duration, and file hash.
pub fn export_generated_wav(
    composition: &GeneratedComposition,
    output_path: &Path,
) -> Result<WavArtifactSummary> {
    let audio_engine = AudioEngine::new();
    audio_engine.write_wav_file(
        output_path,
        &composition.audio_samples,
        composition.preset.audio.sample_rate,
    )
}

/// Convenience function: loads the demo preset, generates a composition, and optionally
/// writes MIDI and/or WAV files.
///
/// Pass `None` for `midi_output` or `wav_output` to skip that export.
pub fn export_demo_artifacts(
    preset_dir: &Path,
    seed: u64,
    midi_output: Option<&Path>,
    wav_output: Option<&Path>,
) -> Result<DemoArtifactSummary> {
    let preset = load_preset(DEMO_PRESET_NAME, preset_dir)?;
    let composition = generate_composition(preset, seed)?;

    let midi = match midi_output {
        Some(path) => Some(export_generated_midi(&composition, path)?),
        None => None,
    };
    let wav = match wav_output {
        Some(path) => Some(export_generated_wav(&composition, path)?),
        None => None,
    };

    Ok(DemoArtifactSummary {
        preset: composition.preset.name.clone(),
        seed,
        trajectory: composition.trajectory_summary.clone(),
        midi,
        wav,
    })
}

/// Encodes a [`MidiModel`] as a Standard MIDI File (Format 0) and writes it to `output_path`.
///
/// Creates parent directories if needed. Returns a [`MidiArtifactSummary`] on success.
/// Fails if the model contains no notes.
pub fn write_midi_file(
    midi_model: &MidiModel,
    mapping: &MidiMappingConfig,
    output_path: &Path,
) -> Result<MidiArtifactSummary> {
    ensure!(!midi_model.notes.is_empty(), "midi model contains no notes");

    if let Some(parent) = output_path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create {}", parent.display()))?;
        }
    }

    let micros_per_beat = 60_000_000u32 / u32::from(mapping.tempo_bpm.max(1));
    let mut scheduled_events: Vec<(u32, u8, TrackEventKind<'static>)> = vec![
        (
            0,
            0,
            TrackEventKind::Meta(MetaMessage::TrackName(b"state-space-music-box")),
        ),
        (
            0,
            1,
            TrackEventKind::Meta(MetaMessage::Tempo(u24::from(micros_per_beat))),
        ),
    ];

    for note in &midi_model.notes {
        let start_tick = beats_to_ticks(note.start_time, mapping.ticks_per_beat);
        let end_tick = beats_to_ticks(
            note.start_time + note.duration.max(1.0 / f64::from(mapping.ticks_per_beat)),
            mapping.ticks_per_beat,
        )
        .max(start_tick + 1);

        scheduled_events.push((
            start_tick,
            2,
            TrackEventKind::Midi {
                channel: u4::from(midi_model.channel.min(15)),
                message: MidiMessage::NoteOn {
                    key: u7::from(note.note.min(127)),
                    vel: u7::from(note.velocity.clamp(1, 127)),
                },
            },
        ));
        scheduled_events.push((
            end_tick,
            1,
            TrackEventKind::Midi {
                channel: u4::from(midi_model.channel.min(15)),
                message: MidiMessage::NoteOff {
                    key: u7::from(note.note.min(127)),
                    vel: u7::from(0),
                },
            },
        ));
    }

    scheduled_events.sort_by(|left, right| left.0.cmp(&right.0).then(left.1.cmp(&right.1)));

    let mut last_tick = 0u32;
    let mut track = Vec::with_capacity(scheduled_events.len() + 1);
    for (tick, _, kind) in scheduled_events {
        track.push(TrackEvent {
            delta: u28::new(tick.saturating_sub(last_tick)),
            kind,
        });
        last_tick = tick;
    }
    track.push(TrackEvent {
        delta: u28::new(0),
        kind: TrackEventKind::Meta(MetaMessage::EndOfTrack),
    });

    let smf = Smf {
        header: Header::new(
            Format::SingleTrack,
            Timing::Metrical(u15::from(mapping.ticks_per_beat.max(1))),
        ),
        tracks: vec![track],
    };
    smf.save(output_path)
        .with_context(|| format!("failed to save MIDI file {}", output_path.display()))?;

    let bytes_written = fs::metadata(output_path)
        .with_context(|| format!("failed to stat {}", output_path.display()))?
        .len();
    let duration_beats = midi_model
        .notes
        .iter()
        .map(|note| note.start_time + note.duration)
        .fold(0.0, f64::max);

    Ok(MidiArtifactSummary {
        path: output_path.to_path_buf(),
        note_count: midi_model.notes.len(),
        duration_beats,
        tempo_bpm: mapping.tempo_bpm,
        bytes_written,
        artifact_hash: crate::governance::sha256_hex(&fs::read(output_path)?),
    })
}

fn map_trajectory_to_midi(
    preset: &RenderPreset,
    trajectory: &[TrajectoryFrame],
    seed: u64,
) -> Result<MidiModel> {
    ensure!(!trajectory.is_empty(), "trajectory contains no frames");

    let outputs: Vec<f64> = trajectory.iter().map(primary_output_value).collect();
    let min_output = outputs.iter().copied().fold(f64::INFINITY, f64::min);
    let max_output = outputs.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let peak_abs_output = outputs.iter().map(|value| value.abs()).fold(0.0, f64::max);

    let mut rng = StdRng::seed_from_u64(seed);
    let beats_total = preset.simulation.duration_seconds.max(0.01)
        * f64::from(preset.midi.tempo_bpm.max(1))
        / 60.0;
    let step_beats = preset.midi.step_beats.max(0.25);
    let step_count = ((beats_total / step_beats).floor() as usize).max(1);
    let mut midi_model = MidiModel::new(
        preset.name.clone(),
        preset.midi.channel,
        preset.midi.default_velocity,
    );

    for step in 0..step_count {
        let frame_index = if step_count == 1 {
            0
        } else {
            ((step as f64 / (step_count - 1) as f64) * (trajectory.len() - 1) as f64).round()
                as usize
        };
        let value = outputs[frame_index];
        let normalized = if (max_output - min_output).abs() < f64::EPSILON {
            0.5
        } else {
            ((value - min_output) / (max_output - min_output)).clamp(0.0, 1.0)
        };
        let target = f64::from(preset.midi.low_note)
            + normalized * f64::from(preset.midi.high_note.saturating_sub(preset.midi.low_note));
        let jitter_bound = i16::from(preset.midi.seed_variation_semitones);
        let jitter = rng.gen_range(-jitter_bound..=jitter_bound);
        let pitch = quantize_to_scale(
            target.round() as i16 + jitter,
            preset.midi.low_note,
            preset.midi.high_note,
            &preset.midi.scale,
            preset.midi.root_note,
        );

        let energy = if peak_abs_output <= f64::EPSILON {
            0.5
        } else {
            (value.abs() / peak_abs_output).clamp(0.0, 1.0)
        };
        let velocity = (i16::from(preset.midi.default_velocity)
            + ((energy - 0.5) * 48.0).round() as i16
            + rng.gen_range(-8..=8))
        .clamp(1, 127) as u8;

        midi_model.add_note(MidiNote::new(
            pitch,
            velocity,
            step as f64 * step_beats,
            step_beats,
        ));
    }

    ensure!(
        !midi_model.notes.is_empty(),
        "generated midi model contains no notes"
    );
    Ok(midi_model)
}

fn quantize_to_scale(target: i16, low_note: u8, high_note: u8, scale: &[u8], root_note: u8) -> u8 {
    let low = i16::from(low_note);
    let high = i16::from(high_note.max(low_note));
    let target = target.clamp(low, high);
    let root_pitch_class = i16::from(root_note % 12);

    let mut best_note = low_note;
    let mut best_distance = i16::MAX;
    for note in low_note..=high_note.max(low_note) {
        let pitch_class = i16::from(note % 12);
        let relative = ((pitch_class - root_pitch_class) + 12) % 12;
        if !scale
            .iter()
            .any(|interval| i16::from(*interval) == relative)
        {
            continue;
        }

        let distance = (i16::from(note) - target).abs();
        if distance < best_distance {
            best_distance = distance;
            best_note = note;
        }
    }

    best_note
}

fn beats_to_ticks(beats: f64, ticks_per_beat: u16) -> u32 {
    (beats.max(0.0) * f64::from(ticks_per_beat.max(1))).round() as u32
}

fn primary_output_value(frame: &TrajectoryFrame) -> f64 {
    frame
        .output
        .first()
        .copied()
        .unwrap_or_else(|| frame.state.first().copied().unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use hound::WavReader;
    use midly::Smf;
    use tempfile::tempdir;

    #[test]
    fn test_demo_preset_generation_is_deterministic() {
        let preset = demo_preset();
        let first = generate_composition(preset.clone(), 7).unwrap();
        let second = generate_composition(preset, 7).unwrap();

        assert_eq!(first.midi_model.notes.len(), second.midi_model.notes.len());
        assert_eq!(first.audio_samples, second.audio_samples);
        assert_eq!(
            first.trajectory_summary.frame_count,
            second.trajectory_summary.frame_count
        );
    }

    #[test]
    fn test_same_seed_writes_identical_midi_and_wav() {
        let dir = tempdir().unwrap();
        let preset = demo_preset();
        let first = generate_composition(preset.clone(), 11).unwrap();
        let second = generate_composition(preset, 11).unwrap();

        let midi_one = dir.path().join("one.mid");
        let midi_two = dir.path().join("two.mid");
        let wav_one = dir.path().join("one.wav");
        let wav_two = dir.path().join("two.wav");

        write_midi_file(&first.midi_model, &first.preset.midi, &midi_one).unwrap();
        write_midi_file(&second.midi_model, &second.preset.midi, &midi_two).unwrap();

        let audio_engine = AudioEngine::new();
        audio_engine
            .write_wav_file(
                &wav_one,
                &first.audio_samples,
                first.preset.audio.sample_rate,
            )
            .unwrap();
        audio_engine
            .write_wav_file(
                &wav_two,
                &second.audio_samples,
                second.preset.audio.sample_rate,
            )
            .unwrap();

        assert_eq!(fs::read(&midi_one).unwrap(), fs::read(&midi_two).unwrap());
        assert_eq!(fs::read(&wav_one).unwrap(), fs::read(&wav_two).unwrap());
    }

    #[test]
    fn test_midi_and_wav_exports_parse() {
        let dir = tempdir().unwrap();
        let composition = generate_composition(demo_preset(), 5).unwrap();
        let midi_path = dir.path().join("demo.mid");
        let wav_path = dir.path().join("demo.wav");

        let midi_summary = write_midi_file(
            &composition.midi_model,
            &composition.preset.midi,
            &midi_path,
        )
        .unwrap();
        let wav_summary = AudioEngine::new()
            .write_wav_file(
                &wav_path,
                &composition.audio_samples,
                composition.preset.audio.sample_rate,
            )
            .unwrap();

        let midi_bytes = fs::read(&midi_path).unwrap();
        let smf = Smf::parse(&midi_bytes).unwrap();
        let wav = WavReader::open(&wav_path).unwrap();

        assert_eq!(midi_summary.note_count, composition.midi_model.notes.len());
        assert_eq!(smf.tracks.len(), 1);
        assert_eq!(
            wav_summary.sample_rate,
            composition.preset.audio.sample_rate
        );
        assert!(wav.duration() > 0);
    }
}
