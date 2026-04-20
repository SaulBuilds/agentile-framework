//! Demonstrate the evaluation workflow: generate multiple seeds, compare, score.
//!
//! Run with:
//!   cargo run --example evaluation_loop

use std::collections::BTreeMap;

use state_space_music_box::generation::{demo_preset, generate_composition, save_preset};
use state_space_music_box::governance::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tmp = tempfile::tempdir()?;
    let preset_dir = tmp.path().join("presets");
    let runtime_dir = tmp.path().join("runtime");

    let mut preset = demo_preset();
    preset.name = "eval-demo".to_string();
    save_preset(&preset, &preset_dir)?;

    println!("Generating 3 compositions with different seeds...\n");

    // Generate compositions with seeds 1, 2, 3
    for seed in 1..=3u64 {
        let comp = generate_composition(demo_preset(), seed)?;
        println!(
            "Seed {seed}: {} notes, {:.1}s, peak output {:.3}",
            comp.midi_model.notes().len(),
            comp.trajectory_summary.duration_seconds,
            comp.trajectory_summary.peak_abs_output,
        );
    }

    // Create a session and render a preview for evaluation
    let session_store = default_session_store_path(&runtime_dir);
    let eval_store = default_evaluation_store_path(&runtime_dir);
    let manifest_dir = default_manifest_dir(&runtime_dir);

    let session = create_session(
        &session_store,
        &preset_dir,
        NewSessionRequest {
            display_name: "Eval Session".to_string(),
            preset_name: "eval-demo".to_string(),
            seed: 1,
            actor_id: "evaluator".to_string(),
        },
    )?;

    let preview = render_session_preview(
        &session_store,
        &preset_dir,
        &runtime_dir,
        &session.session_id,
        "evaluator",
    )?;

    // Submit an evaluation with objective metrics and human scores
    let mut metrics = BTreeMap::new();
    metrics.insert(
        "note_density".to_string(),
        preview.preview.midi.note_count as f64 / 8.0,
    );
    metrics.insert("peak_output".to_string(), 0.85);

    let mut human_scores = BTreeMap::new();
    human_scores.insert("musicality".to_string(), 5u8);
    human_scores.insert("novelty".to_string(), 4u8);

    let mut reward_weights = BTreeMap::new();
    reward_weights.insert("objective".to_string(), 0.4);
    reward_weights.insert("human".to_string(), 0.6);

    let eval = submit_evaluation_record(
        &eval_store,
        &manifest_dir,
        NewEvaluationRecord {
            run_ids: vec![preview.preview.preview_id.clone()],
            objective_metrics: metrics,
            human_scores,
            reward_weights,
            notes: Some("Demo evaluation - sounds interesting".to_string()),
            decision: EvaluationDecision::Promote,
            created_by: "evaluator".to_string(),
        },
    )?;

    println!("\nSubmitted evaluation: {}", eval.evaluation_id);
    println!("  Aggregate score: {:.2}", eval.aggregate_score);
    println!("  Decision: {:?}", eval.decision);

    Ok(())
}
