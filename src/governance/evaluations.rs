use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use anyhow::{bail, ensure, Result};
use clap::ValueEnum;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{
    current_unix_seconds, inspect_run_manifest, list_run_manifests, new_runtime_id,
    read_json_or_default, write_pretty_json, RunManifestRecord,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, ValueEnum, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EvaluationDecision {
    Reject,
    KeepForReference,
    Promote,
    QueueForFurtherSearch,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct EvaluationRecord {
    pub evaluation_id: String,
    pub run_ids: Vec<String>,
    pub objective_metrics: BTreeMap<String, f64>,
    pub human_scores: BTreeMap<String, u8>,
    pub reward_weights: BTreeMap<String, f64>,
    pub aggregate_score: f64,
    pub notes: Option<String>,
    pub decision: EvaluationDecision,
    pub created_by: String,
    pub created_at_unix_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct NewEvaluationRecord {
    pub run_ids: Vec<String>,
    pub objective_metrics: BTreeMap<String, f64>,
    pub human_scores: BTreeMap<String, u8>,
    pub reward_weights: BTreeMap<String, f64>,
    pub notes: Option<String>,
    pub decision: EvaluationDecision,
    pub created_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct RunComparisonEntry {
    pub run_id: String,
    pub action: String,
    pub preset_name: Option<String>,
    pub seed: Option<u64>,
    pub status: super::ActionStatus,
    pub artifact_hashes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct RunComparisonSummary {
    pub runs: Vec<RunComparisonEntry>,
    pub differing_fields: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct ReviewRunSummary {
    pub run_id: String,
    pub evaluation_ids: Vec<String>,
    pub latest_decision: Option<EvaluationDecision>,
    pub best_aggregate_score: Option<f64>,
    pub average_aggregate_score: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct ReviewBundle {
    pub comparison: RunComparisonSummary,
    pub evaluations: Vec<EvaluationRecord>,
    pub runs: Vec<ReviewRunSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ReviewBundleExportSummary {
    pub path: PathBuf,
    pub run_count: usize,
    pub evaluation_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct EvaluationStoreFile {
    version: u32,
    evaluations: Vec<EvaluationRecord>,
}

pub fn submit_evaluation_record(
    store_path: &Path,
    manifest_dir: &Path,
    input: NewEvaluationRecord,
) -> Result<EvaluationRecord> {
    validate_new_evaluation(&input)?;
    for run_id in &input.run_ids {
        inspect_run_manifest(manifest_dir, run_id)?;
    }

    let aggregate_score = compute_aggregate_score(
        &input.objective_metrics,
        &input.human_scores,
        &input.reward_weights,
    )?;
    let record = EvaluationRecord {
        evaluation_id: new_runtime_id("evaluation"),
        run_ids: dedupe_run_ids(&input.run_ids),
        objective_metrics: input.objective_metrics,
        human_scores: input.human_scores,
        reward_weights: input.reward_weights,
        aggregate_score,
        notes: input.notes,
        decision: input.decision,
        created_by: input.created_by,
        created_at_unix_seconds: current_unix_seconds(),
    };

    let mut store = load_store(store_path)?;
    store.evaluations.push(record.clone());
    save_store(store_path, &store)?;
    Ok(record)
}

pub fn inspect_evaluation_record(
    store_path: &Path,
    evaluation_id: &str,
) -> Result<EvaluationRecord> {
    ensure!(
        !evaluation_id.trim().is_empty(),
        "evaluation id cannot be empty"
    );
    let store = load_store(store_path)?;
    store
        .evaluations
        .into_iter()
        .find(|record| record.evaluation_id == evaluation_id)
        .ok_or_else(|| anyhow::anyhow!("evaluation '{}' was not found", evaluation_id))
}

pub fn list_evaluation_records(store_path: &Path) -> Result<Vec<EvaluationRecord>> {
    let mut store = load_store(store_path)?;
    store.evaluations.sort_by(|left, right| {
        left.created_at_unix_seconds
            .cmp(&right.created_at_unix_seconds)
    });
    Ok(store.evaluations)
}

pub fn compare_runs(manifest_dir: &Path, run_ids: &[String]) -> Result<RunComparisonSummary> {
    ensure!(run_ids.len() >= 2, "at least two run ids are required");
    let manifests: Vec<RunManifestRecord> = run_ids
        .iter()
        .map(|run_id| inspect_run_manifest(manifest_dir, run_id))
        .collect::<Result<Vec<_>>>()?;

    let runs: Vec<RunComparisonEntry> = manifests
        .iter()
        .map(|manifest| RunComparisonEntry {
            run_id: manifest.run_id.clone(),
            action: manifest.action.clone(),
            preset_name: manifest.preset_name.clone(),
            seed: manifest.seed,
            status: manifest.status,
            artifact_hashes: manifest
                .artifacts
                .iter()
                .map(|artifact| artifact.sha256.clone())
                .collect(),
        })
        .collect();

    let mut differing_fields = BTreeSet::new();
    if has_differences(runs.iter().map(|run| run.action.as_str())) {
        differing_fields.insert("action".to_string());
    }
    if has_differences(
        runs.iter()
            .map(|run| run.preset_name.as_deref().unwrap_or("")),
    ) {
        differing_fields.insert("preset_name".to_string());
    }
    if has_differences(runs.iter().map(|run| run.seed.unwrap_or_default())) {
        differing_fields.insert("seed".to_string());
    }
    if has_differences(runs.iter().map(|run| run.status)) {
        differing_fields.insert("status".to_string());
    }
    if has_differences(runs.iter().map(|run| run.artifact_hashes.join(","))) {
        differing_fields.insert("artifact_hashes".to_string());
    }

    Ok(RunComparisonSummary {
        runs,
        differing_fields: differing_fields.into_iter().collect(),
    })
}

pub fn build_review_bundle(
    evaluation_store_path: &Path,
    manifest_dir: &Path,
    run_ids: &[String],
) -> Result<ReviewBundle> {
    let comparison = compare_runs(manifest_dir, run_ids)?;
    let run_id_set: BTreeSet<&str> = run_ids.iter().map(String::as_str).collect();
    let evaluations: Vec<EvaluationRecord> = list_evaluation_records(evaluation_store_path)?
        .into_iter()
        .filter(|evaluation| {
            evaluation
                .run_ids
                .iter()
                .any(|run_id| run_id_set.contains(run_id.as_str()))
        })
        .collect();

    let runs = comparison
        .runs
        .iter()
        .map(|run| summarize_run_reviews(&evaluations, &run.run_id))
        .collect();

    Ok(ReviewBundle {
        comparison,
        evaluations,
        runs,
    })
}

pub fn export_review_bundle(
    evaluation_store_path: &Path,
    manifest_dir: &Path,
    run_ids: &[String],
    output_path: &Path,
) -> Result<ReviewBundleExportSummary> {
    let review = build_review_bundle(evaluation_store_path, manifest_dir, run_ids)?;
    write_pretty_json(output_path, &review)?;
    Ok(ReviewBundleExportSummary {
        path: output_path.to_path_buf(),
        run_count: review.runs.len(),
        evaluation_count: review.evaluations.len(),
    })
}

pub fn list_compareable_runs(manifest_dir: &Path) -> Result<Vec<RunManifestRecord>> {
    list_run_manifests(manifest_dir)
}

fn summarize_run_reviews(evaluations: &[EvaluationRecord], run_id: &str) -> ReviewRunSummary {
    let matching: Vec<&EvaluationRecord> = evaluations
        .iter()
        .filter(|evaluation| {
            evaluation
                .run_ids
                .iter()
                .any(|candidate| candidate == run_id)
        })
        .collect();
    let evaluation_ids = matching
        .iter()
        .map(|evaluation| evaluation.evaluation_id.clone())
        .collect::<Vec<_>>();
    let latest_decision = matching.last().map(|evaluation| evaluation.decision);
    let best_aggregate_score = matching
        .iter()
        .map(|evaluation| evaluation.aggregate_score)
        .max_by(f64::total_cmp);
    let average_aggregate_score = (!matching.is_empty()).then(|| {
        matching
            .iter()
            .map(|evaluation| evaluation.aggregate_score)
            .sum::<f64>()
            / matching.len() as f64
    });

    ReviewRunSummary {
        run_id: run_id.to_string(),
        evaluation_ids,
        latest_decision,
        best_aggregate_score,
        average_aggregate_score,
    }
}

fn compute_aggregate_score(
    objective_metrics: &BTreeMap<String, f64>,
    human_scores: &BTreeMap<String, u8>,
    reward_weights: &BTreeMap<String, f64>,
) -> Result<f64> {
    ensure!(!reward_weights.is_empty(), "reward weights cannot be empty");

    let mut total = 0.0;
    let mut total_weight = 0.0;
    for (name, weight) in reward_weights {
        ensure!(
            weight.is_finite() && *weight > 0.0,
            "weight '{name}' must be positive and finite"
        );
        let value = if let Some(score) = human_scores.get(name) {
            f64::from(*score)
        } else if let Some(metric) = objective_metrics.get(name) {
            *metric
        } else {
            bail!("reward weight '{name}' has no matching metric or human score");
        };
        total += value * weight;
        total_weight += weight;
    }
    ensure!(
        total_weight > 0.0,
        "reward weights must sum to more than zero"
    );
    Ok(total / total_weight)
}

fn dedupe_run_ids(run_ids: &[String]) -> Vec<String> {
    let mut seen = BTreeSet::new();
    let mut deduped = Vec::new();
    for run_id in run_ids {
        if seen.insert(run_id.clone()) {
            deduped.push(run_id.clone());
        }
    }
    deduped
}

fn has_differences<T>(mut values: impl Iterator<Item = T>) -> bool
where
    T: PartialEq,
{
    let Some(first) = values.next() else {
        return false;
    };
    values.any(|value| value != first)
}

fn validate_new_evaluation(input: &NewEvaluationRecord) -> Result<()> {
    ensure!(
        !input.created_by.trim().is_empty(),
        "created_by cannot be empty"
    );
    ensure!(!input.run_ids.is_empty(), "run_ids cannot be empty");
    for value in input.objective_metrics.values() {
        ensure!(value.is_finite(), "objective metric values must be finite");
    }
    for (name, score) in &input.human_scores {
        ensure!(
            (1..=7).contains(score),
            "human score '{name}' must be between 1 and 7"
        );
    }
    if input.objective_metrics.is_empty() && input.human_scores.is_empty() {
        bail!("evaluation must include at least one objective metric or human score");
    }
    Ok(())
}

fn load_store(store_path: &Path) -> Result<EvaluationStoreFile> {
    let mut store: EvaluationStoreFile = read_json_or_default(store_path)?;
    if store.version == 0 {
        store.version = 1;
    }
    Ok(store)
}

fn save_store(store_path: &Path, store: &EvaluationStoreFile) -> Result<()> {
    write_pretty_json(store_path, store)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use serde_json::json;
    use tempfile::tempdir;

    use super::*;
    use crate::governance::{
        persist_action_record, ActionStatus, ActionTransport, NewActionRecord,
    };

    fn write_run(runtime_dir: &Path, action: &str, preset_name: &str, seed: u64) -> String {
        let artifact = runtime_dir.join(format!("{action}-{seed}.mid"));
        fs::create_dir_all(runtime_dir).unwrap();
        fs::write(&artifact, b"midi").unwrap();
        persist_action_record(
            runtime_dir,
            NewActionRecord {
                action: action.to_string(),
                actor_id: "tester".to_string(),
                transport: ActionTransport::Cli,
                target: Some(preset_name.to_string()),
                status: ActionStatus::Succeeded,
                input: json!({ "seed": seed }),
                output: Some(json!({ "path": artifact })),
                metadata: None,
                preset_name: Some(preset_name.to_string()),
                preset_hash: Some(format!("hash-{preset_name}")),
                seed: Some(seed),
                approval_ids: Vec::new(),
                artifacts: vec![super::super::ManifestArtifactInput {
                    kind: "midi".to_string(),
                    path: artifact,
                }],
                error_message: None,
            },
        )
        .unwrap()
        .run_id
    }

    #[test]
    fn test_submit_and_list_evaluation() {
        let dir = tempdir().unwrap();
        let runtime_dir = dir.path().join("runtime");
        let manifest_dir = runtime_dir.join("manifests");
        let store_path = runtime_dir.join("evaluations.json");
        let run_id = write_run(&runtime_dir, "generate_midi", "demo", 7);

        let mut objective_metrics = BTreeMap::new();
        objective_metrics.insert("note_density".to_string(), 0.8);
        let mut human_scores = BTreeMap::new();
        human_scores.insert("musicality".to_string(), 6);
        let mut reward_weights = BTreeMap::new();
        reward_weights.insert("musicality".to_string(), 1.0);

        let record = submit_evaluation_record(
            &store_path,
            &manifest_dir,
            NewEvaluationRecord {
                run_ids: vec![run_id.clone()],
                objective_metrics,
                human_scores,
                reward_weights,
                notes: Some("strong candidate".to_string()),
                decision: EvaluationDecision::Promote,
                created_by: "tester".to_string(),
            },
        )
        .unwrap();

        assert_eq!(record.aggregate_score, 6.0);
        assert_eq!(list_evaluation_records(&store_path).unwrap().len(), 1);
        assert_eq!(
            inspect_evaluation_record(&store_path, &record.evaluation_id)
                .unwrap()
                .run_ids,
            vec![run_id]
        );
    }

    #[test]
    fn test_compare_runs_and_invalid_evaluation_rejection() {
        let dir = tempdir().unwrap();
        let runtime_dir = dir.path().join("runtime");
        let manifest_dir = runtime_dir.join("manifests");
        let first = write_run(&runtime_dir, "generate_midi", "demo", 7);
        let second = write_run(&runtime_dir, "generate_audio", "demo-2", 9);

        let comparison = compare_runs(&manifest_dir, &[first.clone(), second.clone()]).unwrap();
        assert_eq!(comparison.runs.len(), 2);
        assert!(comparison.differing_fields.contains(&"action".to_string()));
        assert!(comparison.differing_fields.contains(&"seed".to_string()));

        let error = submit_evaluation_record(
            &runtime_dir.join("evaluations.json"),
            &manifest_dir,
            NewEvaluationRecord {
                run_ids: vec![first],
                objective_metrics: BTreeMap::new(),
                human_scores: BTreeMap::new(),
                reward_weights: BTreeMap::new(),
                notes: None,
                decision: EvaluationDecision::Reject,
                created_by: "tester".to_string(),
            },
        )
        .unwrap_err();
        assert!(error.to_string().contains("at least one objective metric"));
    }

    #[test]
    fn test_build_and_export_review_bundle() {
        let dir = tempdir().unwrap();
        let runtime_dir = dir.path().join("runtime");
        let manifest_dir = runtime_dir.join("manifests");
        let evaluation_store_path = runtime_dir.join("evaluations.json");
        let output_path = runtime_dir.join("reviews/review.json");
        let first = write_run(&runtime_dir, "generate_midi", "demo", 7);
        let second = write_run(&runtime_dir, "generate_audio", "demo", 8);

        submit_evaluation_record(
            &evaluation_store_path,
            &manifest_dir,
            NewEvaluationRecord {
                run_ids: vec![first.clone(), second.clone()],
                objective_metrics: BTreeMap::from([("note_density".to_string(), 0.5)]),
                human_scores: BTreeMap::from([("musicality".to_string(), 6)]),
                reward_weights: BTreeMap::from([("musicality".to_string(), 1.0)]),
                notes: Some("worth keeping".to_string()),
                decision: EvaluationDecision::KeepForReference,
                created_by: "tester".to_string(),
            },
        )
        .unwrap();

        let review = build_review_bundle(
            &evaluation_store_path,
            &manifest_dir,
            &[first.clone(), second.clone()],
        )
        .unwrap();
        assert_eq!(review.comparison.runs.len(), 2);
        assert_eq!(review.evaluations.len(), 1);
        assert_eq!(review.runs.len(), 2);
        assert_eq!(
            review.runs[0].latest_decision,
            Some(EvaluationDecision::KeepForReference)
        );

        let export = export_review_bundle(
            &evaluation_store_path,
            &manifest_dir,
            &[first, second],
            &output_path,
        )
        .unwrap();
        assert_eq!(export.run_count, 2);
        assert!(output_path.exists());
    }
}
