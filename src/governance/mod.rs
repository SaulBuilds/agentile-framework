use std::fs;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result};
use serde::de::DeserializeOwned;
use serde::Serialize;
use sha2::{Digest, Sha256};

pub mod approvals;
pub mod audit;
pub mod creative;
pub mod datasets;
pub mod daw;
pub mod evaluations;
pub mod harness;
pub mod policy;
pub mod realtime;
pub mod scheduler;
pub mod sessions;
pub mod snapshots;

pub use approvals::*;
pub use audit::*;
pub use creative::*;
pub use datasets::*;
pub use daw::*;
pub use evaluations::*;
pub use harness::*;
pub use policy::*;
pub use realtime::*;
pub use scheduler::*;
pub use sessions::*;
pub use snapshots::*;

const DEFAULT_RUNTIME_DIR: &str = ".agentile/runtime";

static ID_COUNTER: AtomicU64 = AtomicU64::new(1);

pub fn default_runtime_dir() -> PathBuf {
    PathBuf::from(DEFAULT_RUNTIME_DIR)
}

pub fn default_dataset_registry_path(runtime_dir: &Path) -> PathBuf {
    runtime_dir.join("datasets.json")
}

pub fn default_approval_store_path(runtime_dir: &Path) -> PathBuf {
    runtime_dir.join("approvals.json")
}

pub fn default_snapshot_dir(runtime_dir: &Path) -> PathBuf {
    runtime_dir.join("snapshots")
}

pub fn default_manifest_dir(runtime_dir: &Path) -> PathBuf {
    runtime_dir.join("manifests")
}

pub fn default_audit_log_path(runtime_dir: &Path) -> PathBuf {
    runtime_dir.join("audit-log.jsonl")
}

pub fn default_session_store_path(runtime_dir: &Path) -> PathBuf {
    runtime_dir.join("sessions.json")
}

pub fn default_evaluation_store_path(runtime_dir: &Path) -> PathBuf {
    runtime_dir.join("evaluations.json")
}

pub fn default_preview_dir(runtime_dir: &Path) -> PathBuf {
    runtime_dir.join("previews")
}

pub fn default_review_dir(runtime_dir: &Path) -> PathBuf {
    runtime_dir.join("reviews")
}

pub fn default_daw_store_path(runtime_dir: &Path) -> PathBuf {
    runtime_dir.join("decks.json")
}

pub fn default_harness_store_path(runtime_dir: &Path) -> PathBuf {
    runtime_dir.join("harness.json")
}

pub fn default_realtime_store_path(runtime_dir: &Path) -> PathBuf {
    runtime_dir.join("realtime-adapters.json")
}

pub fn default_scheduler_store_path(runtime_dir: &Path) -> PathBuf {
    runtime_dir.join("jobs.json")
}

pub fn default_scheduler_export_dir(runtime_dir: &Path) -> PathBuf {
    runtime_dir.join("scheduler-exports")
}

pub(crate) fn current_unix_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

pub(crate) fn current_unix_milliseconds() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}

pub(crate) fn new_runtime_id(prefix: &str) -> String {
    let counter = ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!(
        "{prefix}-{}-{}-{counter}",
        current_unix_milliseconds(),
        std::process::id()
    )
}

pub(crate) fn sha256_hex(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

pub(crate) fn read_json_or_default<T>(path: &Path) -> Result<T>
where
    T: DeserializeOwned + Default,
{
    if !path.exists() {
        return Ok(T::default());
    }

    let raw = fs::read_to_string(path)
        .with_context(|| format!("failed to read JSON file {}", path.display()))?;
    serde_json::from_str(&raw)
        .with_context(|| format!("failed to parse JSON file {}", path.display()))
}

pub(crate) fn write_pretty_json<T>(path: &Path, value: &T) -> Result<()>
where
    T: Serialize,
{
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create directory {}", parent.display()))?;
    }
    let raw = serde_json::to_string_pretty(value)?;
    fs::write(path, raw).with_context(|| format!("failed to write {}", path.display()))?;
    Ok(())
}

pub(crate) fn append_json_line<T>(path: &Path, value: &T) -> Result<()>
where
    T: Serialize,
{
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create directory {}", parent.display()))?;
    }

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .with_context(|| format!("failed to open {}", path.display()))?;
    serde_json::to_writer(&mut file, value)?;
    file.write_all(b"\n")
        .with_context(|| format!("failed to append {}", path.display()))?;
    Ok(())
}

pub(crate) fn read_json_lines<T>(path: &Path) -> Result<Vec<T>>
where
    T: DeserializeOwned,
{
    if !path.exists() {
        return Ok(Vec::new());
    }

    let file =
        fs::File::open(path).with_context(|| format!("failed to open {}", path.display()))?;
    let reader = BufReader::new(file);
    let mut values = Vec::new();

    for (index, line) in reader.lines().enumerate() {
        let line = line.with_context(|| format!("failed to read line {}", index + 1))?;
        if line.trim().is_empty() {
            continue;
        }
        let value = serde_json::from_str(&line).with_context(|| {
            format!(
                "failed to parse JSON line {} in {}",
                index + 1,
                path.display()
            )
        })?;
        values.push(value);
    }

    Ok(values)
}
