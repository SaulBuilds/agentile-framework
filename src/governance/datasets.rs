use std::path::{Path, PathBuf};

use anyhow::{bail, ensure, Result};
use clap::ValueEnum;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{current_unix_seconds, read_json_or_default, write_pretty_json};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, ValueEnum, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PolicyStatus {
    Allowed,
    Restricted,
    Unknown,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, ValueEnum, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ApprovedUseClass {
    ProductionAllowed,
    ResearchOnly,
    LicenseReviewRequired,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, ValueEnum, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DatasetUseContext {
    Production,
    Research,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ChecksumEntry {
    pub relative_path: String,
    pub sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct NewDatasetRecord {
    pub dataset_id: String,
    pub display_name: String,
    pub source_url: String,
    pub citation: Option<String>,
    pub license_name: String,
    pub commercial_use_status: PolicyStatus,
    pub redistribution_status: PolicyStatus,
    pub approved_use_class: ApprovedUseClass,
    pub checksum_manifest: Vec<ChecksumEntry>,
    pub local_storage_path: PathBuf,
    pub dataset_version: String,
    pub split_policy: Option<String>,
    pub transform_pipeline_hash: Option<String>,
    pub parent_datasets: Vec<String>,
    pub operator_approval_id: String,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct DatasetRecord {
    pub dataset_id: String,
    pub display_name: String,
    pub source_url: String,
    pub citation: Option<String>,
    pub license_name: String,
    pub commercial_use_status: PolicyStatus,
    pub redistribution_status: PolicyStatus,
    pub approved_use_class: ApprovedUseClass,
    pub checksum_manifest: Vec<ChecksumEntry>,
    pub local_storage_path: PathBuf,
    pub dataset_version: String,
    pub split_policy: Option<String>,
    pub transform_pipeline_hash: Option<String>,
    pub parent_datasets: Vec<String>,
    pub operator_approval_id: String,
    pub notes: Option<String>,
    pub created_at_unix_seconds: u64,
    pub updated_at_unix_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct DatasetRegistryFile {
    version: u32,
    datasets: Vec<DatasetRecord>,
}

pub fn register_dataset_record(
    registry_path: &Path,
    input: NewDatasetRecord,
) -> Result<DatasetRecord> {
    validate_new_dataset_record(&input)?;

    let mut registry = load_registry(registry_path)?;
    if registry
        .datasets
        .iter()
        .any(|record| record.dataset_id == input.dataset_id)
    {
        bail!("dataset '{}' already exists", input.dataset_id);
    }

    let now = current_unix_seconds();
    let record = DatasetRecord {
        dataset_id: input.dataset_id,
        display_name: input.display_name,
        source_url: input.source_url,
        citation: input.citation,
        license_name: input.license_name,
        commercial_use_status: input.commercial_use_status,
        redistribution_status: input.redistribution_status,
        approved_use_class: input.approved_use_class,
        checksum_manifest: input.checksum_manifest,
        local_storage_path: input.local_storage_path,
        dataset_version: input.dataset_version,
        split_policy: input.split_policy,
        transform_pipeline_hash: input.transform_pipeline_hash,
        parent_datasets: input.parent_datasets,
        operator_approval_id: input.operator_approval_id,
        notes: input.notes,
        created_at_unix_seconds: now,
        updated_at_unix_seconds: now,
    };

    registry.datasets.push(record.clone());
    registry
        .datasets
        .sort_by(|left, right| left.dataset_id.cmp(&right.dataset_id));
    save_registry(registry_path, &registry)?;

    Ok(record)
}

pub fn list_dataset_records(registry_path: &Path) -> Result<Vec<DatasetRecord>> {
    let mut registry = load_registry(registry_path)?;
    registry
        .datasets
        .sort_by(|left, right| left.dataset_id.cmp(&right.dataset_id));
    Ok(registry.datasets)
}

pub fn inspect_dataset_record(registry_path: &Path, dataset_id: &str) -> Result<DatasetRecord> {
    let registry = load_registry(registry_path)?;
    registry
        .datasets
        .into_iter()
        .find(|record| record.dataset_id == dataset_id)
        .ok_or_else(|| anyhow::anyhow!("dataset '{}' was not found", dataset_id))
}

pub fn ensure_dataset_use_allowed(
    record: &DatasetRecord,
    context: DatasetUseContext,
) -> Result<()> {
    match (record.approved_use_class, context) {
        (ApprovedUseClass::ProductionAllowed, _) => Ok(()),
        (ApprovedUseClass::ResearchOnly, DatasetUseContext::Research) => Ok(()),
        (ApprovedUseClass::ResearchOnly, DatasetUseContext::Production) => bail!(
            "dataset '{}' is marked research_only and cannot be used in production",
            record.dataset_id
        ),
        (ApprovedUseClass::LicenseReviewRequired, _) => bail!(
            "dataset '{}' requires license review before use",
            record.dataset_id
        ),
    }
}

fn load_registry(registry_path: &Path) -> Result<DatasetRegistryFile> {
    let mut registry: DatasetRegistryFile = read_json_or_default(registry_path)?;
    if registry.version == 0 {
        registry.version = 1;
    }
    Ok(registry)
}

fn save_registry(registry_path: &Path, registry: &DatasetRegistryFile) -> Result<()> {
    write_pretty_json(registry_path, registry)
}

fn validate_new_dataset_record(input: &NewDatasetRecord) -> Result<()> {
    ensure!(
        is_valid_identifier(&input.dataset_id),
        "dataset id '{}' must use ASCII letters, digits, '.', '-', or '_'",
        input.dataset_id
    );
    ensure!(
        !input.display_name.trim().is_empty(),
        "display name cannot be empty"
    );
    ensure!(
        has_supported_url_scheme(&input.source_url),
        "source url '{}' must start with http://, https://, or file://",
        input.source_url
    );
    ensure!(
        !input.license_name.trim().is_empty(),
        "license name cannot be empty"
    );
    ensure!(
        !input.operator_approval_id.trim().is_empty(),
        "operator approval id cannot be empty"
    );
    ensure!(
        !input.dataset_version.trim().is_empty(),
        "dataset version cannot be empty"
    );
    ensure!(
        !input.local_storage_path.as_os_str().is_empty(),
        "local storage path cannot be empty"
    );
    ensure!(
        !input.checksum_manifest.is_empty(),
        "checksum manifest cannot be empty"
    );

    for checksum in &input.checksum_manifest {
        ensure!(
            !checksum.relative_path.trim().is_empty(),
            "checksum relative_path cannot be empty"
        );
        ensure!(
            checksum.sha256.len() == 64 && checksum.sha256.chars().all(|ch| ch.is_ascii_hexdigit()),
            "checksum for '{}' must be a 64-character hex sha256 value",
            checksum.relative_path
        );
    }

    Ok(())
}

fn is_valid_identifier(value: &str) -> bool {
    !value.is_empty()
        && value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '.' | '-' | '_'))
}

fn has_supported_url_scheme(value: &str) -> bool {
    ["http://", "https://", "file://"]
        .iter()
        .any(|prefix| value.starts_with(prefix))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn valid_record() -> NewDatasetRecord {
        NewDatasetRecord {
            dataset_id: "pdmx".to_string(),
            display_name: "PDMX".to_string(),
            source_url: "https://example.com/pdmx".to_string(),
            citation: Some("Example citation".to_string()),
            license_name: "CC-BY-4.0".to_string(),
            commercial_use_status: PolicyStatus::Allowed,
            redistribution_status: PolicyStatus::Allowed,
            approved_use_class: ApprovedUseClass::ProductionAllowed,
            checksum_manifest: vec![ChecksumEntry {
                relative_path: "archive.tar.gz".to_string(),
                sha256: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
                    .to_string(),
            }],
            local_storage_path: PathBuf::from("/datasets/pdmx"),
            dataset_version: "v1".to_string(),
            split_policy: Some("train/valid/test".to_string()),
            transform_pipeline_hash: Some("pipeline-hash".to_string()),
            parent_datasets: vec![],
            operator_approval_id: "approval-1".to_string(),
            notes: Some("ready".to_string()),
        }
    }

    #[test]
    fn test_register_list_and_inspect_dataset() {
        let dir = tempdir().unwrap();
        let registry_path = dir.path().join("datasets.json");

        let created = register_dataset_record(&registry_path, valid_record()).unwrap();
        let listed = list_dataset_records(&registry_path).unwrap();
        let inspected = inspect_dataset_record(&registry_path, "pdmx").unwrap();

        assert_eq!(created.dataset_id, "pdmx");
        assert_eq!(listed.len(), 1);
        assert_eq!(inspected.display_name, "PDMX");
    }

    #[test]
    fn test_register_dataset_rejects_invalid_record() {
        let dir = tempdir().unwrap();
        let registry_path = dir.path().join("datasets.json");
        let mut input = valid_record();
        input.source_url = "ftp://example.com".to_string();

        let error = register_dataset_record(&registry_path, input).unwrap_err();
        assert!(error.to_string().contains("must start with"));
    }

    #[test]
    fn test_dataset_use_policy_blocks_research_only_in_production() {
        let mut record = valid_record();
        record.approved_use_class = ApprovedUseClass::ResearchOnly;
        let record = DatasetRecord {
            created_at_unix_seconds: 1,
            updated_at_unix_seconds: 1,
            ..register_dataset_record(&tempdir().unwrap().path().join("unused.json"), record)
                .unwrap()
        };

        let error = ensure_dataset_use_allowed(&record, DatasetUseContext::Production).unwrap_err();
        assert!(error.to_string().contains("research_only"));
    }
}
