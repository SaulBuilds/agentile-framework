//! Creative tools for agents: preset patching and parameter sweeps.
//!
//! These tools let agents explore the parameter space programmatically
//! rather than manually calling generate+evaluate in a loop.

use std::path::Path;

use anyhow::{ensure, Result};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::generation::{generate_composition, load_preset, save_preset, TrajectorySummary};

use super::{
    create_preset_snapshot, current_unix_seconds, default_snapshot_dir, new_runtime_id,
    read_json_or_default, write_pretty_json,
};

/// A request to patch specific fields of a preset's configuration.
///
/// Only non-null fields are applied. The original preset is snapshotted
/// before mutation for rollback safety.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct PresetPatchRequest {
    /// Name of the preset to patch.
    pub preset_name: String,
    /// Who is making this change (used in audit trail).
    pub actor_id: String,
    /// Reason for the patch (stored in snapshot).
    pub reason: String,
    /// New tempo in BPM (overrides midi_mapping.tempo_bpm).
    pub tempo_bpm: Option<u16>,
    /// New seed variation range in semitones.
    pub seed_variation_semitones: Option<u8>,
    /// New lowest MIDI note.
    pub low_note: Option<u8>,
    /// New highest MIDI note.
    pub high_note: Option<u8>,
    /// New step duration in beats.
    pub step_beats: Option<f64>,
    /// New simulation duration in seconds.
    pub duration_seconds: Option<f64>,
    /// New peak limiter threshold.
    pub peak_limit: Option<f32>,
    /// New root note for scale quantization.
    pub root_note: Option<u8>,
    /// New scale intervals.
    pub scale: Option<Vec<u8>>,
}

/// Result of applying a preset patch.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct PresetPatchResult {
    pub preset_name: String,
    pub snapshot_id: String,
    pub changed_fields: Vec<String>,
    pub patched_at_unix_seconds: u64,
}

/// Apply a diff-based patch to a preset file with automatic snapshotting.
pub fn apply_preset_patch(
    preset_dir: &Path,
    runtime_dir: &Path,
    request: PresetPatchRequest,
) -> Result<PresetPatchResult> {
    ensure!(
        !request.preset_name.trim().is_empty(),
        "preset name cannot be empty"
    );
    ensure!(
        !request.actor_id.trim().is_empty(),
        "actor id cannot be empty"
    );

    // Load and snapshot before mutation
    let mut preset = load_preset(&request.preset_name, preset_dir)?;
    let snapshot = create_preset_snapshot(
        &default_snapshot_dir(runtime_dir),
        preset_dir,
        &request.preset_name,
        &request.reason,
        Some(&request.actor_id),
    )?;

    let mut changed = Vec::new();

    if let Some(tempo) = request.tempo_bpm {
        preset.midi.tempo_bpm = tempo;
        changed.push("midi_mapping.tempo_bpm".to_string());
    }
    if let Some(sv) = request.seed_variation_semitones {
        preset.midi.seed_variation_semitones = sv;
        changed.push("midi_mapping.seed_variation_semitones".to_string());
    }
    if let Some(ln) = request.low_note {
        preset.midi.low_note = ln;
        changed.push("midi_mapping.low_note".to_string());
    }
    if let Some(hn) = request.high_note {
        preset.midi.high_note = hn;
        changed.push("midi_mapping.high_note".to_string());
    }
    if let Some(sb) = request.step_beats {
        preset.midi.step_beats = sb;
        changed.push("midi_mapping.step_beats".to_string());
    }
    if let Some(dur) = request.duration_seconds {
        preset.simulation.duration_seconds = dur;
        changed.push("simulation.duration_seconds".to_string());
    }
    if let Some(pl) = request.peak_limit {
        preset.audio.peak_limit = pl;
        changed.push("audio.peak_limit".to_string());
    }
    if let Some(rn) = request.root_note {
        preset.midi.root_note = rn;
        changed.push("midi_mapping.root_note".to_string());
    }
    if let Some(ref sc) = request.scale {
        preset.midi.scale = sc.clone();
        changed.push("midi_mapping.scale".to_string());
    }

    ensure!(
        !changed.is_empty(),
        "patch request did not change any fields"
    );

    save_preset(&preset, preset_dir)?;

    Ok(PresetPatchResult {
        preset_name: request.preset_name,
        snapshot_id: snapshot.snapshot_id,
        changed_fields: changed,
        patched_at_unix_seconds: current_unix_seconds(),
    })
}

/// A request to sweep across multiple seeds and compare results.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct ParameterSweepRequest {
    /// Preset to use for all sweep runs.
    pub preset_name: String,
    /// List of seeds to try. Each seed produces one composition.
    pub seeds: Vec<u64>,
    /// Who is running the sweep (for audit).
    pub actor_id: String,
}

/// One entry in a parameter sweep result.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct SweepEntry {
    pub seed: u64,
    pub note_count: usize,
    pub trajectory_summary: TrajectorySummary,
}

/// Result of a parameter sweep.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct ParameterSweepResult {
    pub sweep_id: String,
    pub preset_name: String,
    pub entries: Vec<SweepEntry>,
    /// Entries sorted by peak_abs_output descending -- most dynamic first.
    pub ranked_seeds: Vec<u64>,
    pub created_at_unix_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct SweepStoreFile {
    version: u32,
    sweeps: Vec<ParameterSweepResult>,
}

/// Run a multi-seed parameter sweep, generating a composition for each seed
/// and ranking them by trajectory dynamics.
pub fn run_parameter_sweep(
    preset_dir: &Path,
    runtime_dir: &Path,
    request: ParameterSweepRequest,
) -> Result<ParameterSweepResult> {
    ensure!(
        !request.preset_name.trim().is_empty(),
        "preset name cannot be empty"
    );
    ensure!(!request.seeds.is_empty(), "seeds list cannot be empty");
    ensure!(
        request.seeds.len() <= 50,
        "sweep is limited to 50 seeds per run"
    );

    let preset = load_preset(&request.preset_name, preset_dir)?;

    let mut entries = Vec::new();
    for &seed in &request.seeds {
        let comp = generate_composition(preset.clone(), seed)?;
        entries.push(SweepEntry {
            seed,
            note_count: comp.midi_model.notes().len(),
            trajectory_summary: comp.trajectory_summary,
        });
    }

    // Rank by peak_abs_output descending (most dynamic first)
    let mut ranked = entries.clone();
    ranked.sort_by(|a, b| {
        b.trajectory_summary
            .peak_abs_output
            .partial_cmp(&a.trajectory_summary.peak_abs_output)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    let ranked_seeds: Vec<u64> = ranked.iter().map(|e| e.seed).collect();

    let result = ParameterSweepResult {
        sweep_id: new_runtime_id("sweep"),
        preset_name: request.preset_name,
        entries,
        ranked_seeds,
        created_at_unix_seconds: current_unix_seconds(),
    };

    // Persist sweep result
    let store_path = runtime_dir.join("sweeps.json");
    let mut store: SweepStoreFile = read_json_or_default(&store_path)?;
    if store.version == 0 {
        store.version = 1;
    }
    store.sweeps.push(result.clone());
    write_pretty_json(&store_path, &store)?;

    Ok(result)
}

/// List stored sweep results.
pub fn list_sweeps(runtime_dir: &Path) -> Result<Vec<ParameterSweepResult>> {
    let store_path = runtime_dir.join("sweeps.json");
    let store: SweepStoreFile = read_json_or_default(&store_path)?;
    Ok(store.sweeps)
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;
    use crate::generation::{demo_preset, save_preset};

    #[test]
    fn test_preset_patch_applies_and_snapshots() {
        let dir = tempdir().unwrap();
        let preset_dir = dir.path().join("presets");
        let runtime_dir = dir.path().join("runtime");

        let mut preset = demo_preset();
        preset.name = "patch-test".to_string();
        save_preset(&preset, &preset_dir).unwrap();

        let result = apply_preset_patch(
            &preset_dir,
            &runtime_dir,
            PresetPatchRequest {
                preset_name: "patch-test".to_string(),
                actor_id: "tester".to_string(),
                reason: "test patch".to_string(),
                tempo_bpm: Some(160),
                low_note: Some(48),
                high_note: None,
                seed_variation_semitones: None,
                step_beats: None,
                duration_seconds: None,
                peak_limit: None,
                root_note: None,
                scale: None,
            },
        )
        .unwrap();

        assert_eq!(result.changed_fields.len(), 2);
        assert!(result
            .changed_fields
            .contains(&"midi_mapping.tempo_bpm".to_string()));
        assert!(result
            .changed_fields
            .contains(&"midi_mapping.low_note".to_string()));

        // Verify the preset was actually changed
        let reloaded = load_preset("patch-test", &preset_dir).unwrap();
        assert_eq!(reloaded.midi.tempo_bpm, 160);
        assert_eq!(reloaded.midi.low_note, 48);
    }

    #[test]
    fn test_parameter_sweep_ranks_by_dynamics() {
        let dir = tempdir().unwrap();
        let preset_dir = dir.path().join("presets");
        let runtime_dir = dir.path().join("runtime");

        let mut preset = demo_preset();
        preset.name = "sweep-test".to_string();
        save_preset(&preset, &preset_dir).unwrap();

        let result = run_parameter_sweep(
            &preset_dir,
            &runtime_dir,
            ParameterSweepRequest {
                preset_name: "sweep-test".to_string(),
                seeds: vec![1, 2, 3, 4, 5],
                actor_id: "tester".to_string(),
            },
        )
        .unwrap();

        assert_eq!(result.entries.len(), 5);
        assert_eq!(result.ranked_seeds.len(), 5);
        // All entries should have positive note counts
        for entry in &result.entries {
            assert!(entry.note_count > 0);
        }

        // Verify persistence
        let sweeps = list_sweeps(&runtime_dir).unwrap();
        assert_eq!(sweeps.len(), 1);
    }
}
