use std::f64::consts::TAU;
use std::path::Path;

use anyhow::Result;
use hound::{SampleFormat, WavSpec, WavWriter};
use nalgebra::DVector;
use tracing::info;

use crate::generation::WavArtifactSummary;
use crate::midi_model::MidiModel;
use crate::state_space::StateSpaceSystem;

/// Audio engine for generating sound buffers from state-space systems.
#[derive(Debug, Clone)]
pub struct AudioEngine {
    /// Sample rate for audio generation (Hz)
    pub sample_rate: u32,
    /// Buffer size for audio generation
    pub buffer_size: usize,
}

impl Default for AudioEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioEngine {
    /// Create a new audio engine with default settings.
    pub fn new() -> Self {
        info!("Creating audio engine");
        Self {
            sample_rate: 44_100,
            buffer_size: 512,
        }
    }

    /// Start the audio engine.
    pub fn start(&mut self) -> Result<()> {
        info!("Starting audio engine");
        Ok(())
    }

    /// Stop the audio engine.
    pub fn stop(&mut self) -> Result<()> {
        info!("Stopping audio engine");
        Ok(())
    }

    /// Generate a mono audio buffer from a state-space system.
    ///
    /// This path is intentionally deterministic so it can be used in tests and
    /// as a stable SDK baseline while richer rendering paths are still planned.
    pub fn generate_audio_from_state_space(
        &mut self,
        system: &StateSpaceSystem,
        duration: f64,
        sample_rate: u32,
    ) -> Result<Vec<f32>> {
        info!(
            "Generating audio from state-space system for {} seconds",
            duration
        );

        let duration = duration.max(0.0);
        let num_samples = (duration * f64::from(sample_rate)).round() as usize;
        let num_samples = num_samples.max(1);
        let integration_dt = system.dt.unwrap_or_else(|| 1.0 / f64::from(sample_rate));

        let state_dim = system.a.nrows();
        let input_dim = system.b.ncols();
        let output_dim = system.c.nrows();

        let mut audio_buffer = Vec::with_capacity(num_samples);
        let mut state = DVector::zeros(state_dim);
        if state_dim > 0 {
            state[0] = 1.0;
        }
        let input = DVector::zeros(input_dim);

        for _ in 0..num_samples {
            let next_state = if system.dt.is_some() {
                system
                    .predict(&state, &input)
                    .map_err(|err| anyhow::anyhow!(err.to_string()))?
            } else {
                let derivative = system
                    .predict(&state, &input)
                    .map_err(|err| anyhow::anyhow!(err.to_string()))?;
                &state + derivative * integration_dt
            };

            let output = system
                .output(&next_state, &input)
                .map_err(|err| anyhow::anyhow!(err.to_string()))?;
            let raw_sample = if output_dim > 0 {
                output[0]
            } else if state_dim > 0 {
                next_state[0]
            } else {
                0.0
            };

            audio_buffer.push(raw_sample.clamp(-1.0, 1.0) as f32);
            state = next_state;
        }

        Ok(audio_buffer)
    }

    /// Render a MIDI model to a mono audio buffer using a built-in sine synth.
    pub fn render_midi_model(
        &self,
        midi_model: &MidiModel,
        tempo_bpm: u16,
        sample_rate: u32,
        attack_seconds: f64,
        release_seconds: f64,
        peak_limit: f32,
    ) -> Vec<f32> {
        let total_beats = midi_model
            .notes
            .iter()
            .map(|note| note.start_time + note.duration)
            .fold(0.0, f64::max);
        let duration_seconds = total_beats * 60.0 / f64::from(tempo_bpm.max(1));
        let sample_count =
            (duration_seconds.max(0.01) * f64::from(sample_rate.max(1))).ceil() as usize;
        let mut buffer = vec![0.0f32; sample_count.max(1)];

        for note in &midi_model.notes {
            let start_seconds = note.start_time * 60.0 / f64::from(tempo_bpm.max(1));
            let duration_seconds = note.duration.max(0.01) * 60.0 / f64::from(tempo_bpm.max(1));
            let end_seconds = start_seconds + duration_seconds;
            let start_index = (start_seconds * f64::from(sample_rate.max(1)))
                .floor()
                .max(0.0) as usize;
            let end_index = (end_seconds * f64::from(sample_rate.max(1)))
                .ceil()
                .max(1.0) as usize;
            let frequency = 440.0 * 2f64.powf((f64::from(note.note) - 69.0) / 12.0);
            let amplitude = f32::from(note.velocity.max(1)) / 127.0;

            for sample_index in start_index..end_index.min(buffer.len()) {
                let time = sample_index as f64 / f64::from(sample_rate.max(1));
                let note_time = time - start_seconds;
                let remaining = end_seconds - time;
                let attack = if attack_seconds > 0.0 && note_time < attack_seconds {
                    (note_time / attack_seconds).clamp(0.0, 1.0)
                } else {
                    1.0
                };
                let release = if release_seconds > 0.0 && remaining < release_seconds {
                    (remaining / release_seconds).clamp(0.0, 1.0)
                } else {
                    1.0
                };
                let envelope = attack.min(release) as f32;
                let sample = (TAU * frequency * note_time).sin() as f32;
                buffer[sample_index] += sample * amplitude * envelope;
            }
        }

        let peak = buffer
            .iter()
            .map(|sample| sample.abs())
            .fold(0.0f32, f32::max);
        if peak > peak_limit.max(0.01) {
            let scale = peak_limit / peak;
            for sample in &mut buffer {
                *sample *= scale;
            }
        }

        buffer
            .into_iter()
            .map(|sample| sample.clamp(-peak_limit, peak_limit))
            .collect()
    }

    /// Write a mono WAV file from floating point samples.
    pub fn write_wav_file<P: AsRef<Path>>(
        &self,
        output_path: P,
        samples: &[f32],
        sample_rate: u32,
    ) -> Result<WavArtifactSummary> {
        let output_path = output_path.as_ref();
        if let Some(parent) = output_path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)?;
            }
        }

        let spec = WavSpec {
            channels: 1,
            sample_rate,
            bits_per_sample: 16,
            sample_format: SampleFormat::Int,
        };
        let mut writer = WavWriter::create(output_path, spec)?;
        for sample in samples {
            let sample = sample.clamp(-1.0, 1.0);
            writer.write_sample((sample * f32::from(i16::MAX)).round() as i16)?;
        }
        writer.finalize()?;

        let peak_amplitude = samples
            .iter()
            .map(|sample| sample.abs())
            .fold(0.0f32, f32::max);
        let duration_seconds = if sample_rate == 0 {
            0.0
        } else {
            samples.len() as f64 / f64::from(sample_rate)
        };

        Ok(WavArtifactSummary {
            path: output_path.to_path_buf(),
            sample_count: samples.len(),
            duration_seconds,
            peak_amplitude,
            sample_rate,
            artifact_hash: crate::governance::sha256_hex(&std::fs::read(output_path)?),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::midi_model::MidiNote;
    use nalgebra::DMatrix;
    use tempfile::tempdir;

    #[test]
    fn test_audio_engine_creation() {
        let engine = AudioEngine::new();
        assert_eq!(engine.sample_rate, 44_100);
        assert_eq!(engine.buffer_size, 512);
    }

    #[test]
    fn test_generate_audio_simple_system() {
        let a = DMatrix::from_row_slice(2, 2, &[1.0, 0.01, -0.01, 0.999]);
        let b = DMatrix::zeros(2, 0);
        let c = DMatrix::from_row_slice(1, 2, &[1.0, 0.0]);
        let d = DMatrix::zeros(1, 0);
        let system = StateSpaceSystem::new(a, b, c, d, Some(0.01)).unwrap();

        let mut engine = AudioEngine::new();
        let buffer = engine
            .generate_audio_from_state_space(&system, 0.1, 1_000)
            .unwrap();

        assert_eq!(buffer.len(), 100);
        assert!(buffer.iter().any(|sample| sample.abs() > 0.0));
        assert!(buffer.iter().all(|sample| sample.is_finite()));
        assert!(buffer.iter().all(|sample| (-1.0..=1.0).contains(sample)));
    }

    #[test]
    fn test_render_midi_model_produces_non_silent_audio() {
        let mut midi_model = MidiModel::new("demo".to_string(), 0, 96);
        midi_model.add_note(MidiNote::new(60, 100, 0.0, 1.0));
        midi_model.add_note(MidiNote::new(64, 90, 1.0, 1.0));

        let engine = AudioEngine::new();
        let buffer = engine.render_midi_model(&midi_model, 120, 8_000, 0.01, 0.05, 0.8);

        assert!(!buffer.is_empty());
        assert!(buffer.iter().any(|sample| sample.abs() > 0.0));
        assert!(buffer.iter().all(|sample| sample.abs() <= 0.8));
    }

    #[test]
    fn test_write_wav_file_round_trips() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("demo.wav");
        let samples = vec![0.0, 0.25, -0.25, 0.5, -0.5];

        let summary = AudioEngine::new()
            .write_wav_file(&path, &samples, 8_000)
            .unwrap();

        assert_eq!(summary.sample_count, samples.len());
        assert_eq!(summary.sample_rate, 8_000);
        assert!(path.exists());
    }
}
