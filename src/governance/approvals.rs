use std::path::Path;

use anyhow::{bail, ensure, Result};
use clap::ValueEnum;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{current_unix_seconds, new_runtime_id, read_json_or_default, write_pretty_json};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, ValueEnum, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ActionRisk {
    Low,
    Medium,
    ApprovalRequired,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, ValueEnum, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalDecisionKind {
    Approve,
    Deny,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalStatus {
    Pending,
    Approved,
    Denied,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct NewApprovalRequest {
    pub action_scope: String,
    pub target: String,
    pub requested_by: String,
    pub reason: String,
    pub risk: ActionRisk,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ApprovalTokenRecord {
    pub token_id: String,
    pub approval_id: String,
    pub action_scope: String,
    pub target: String,
    pub issued_at_unix_seconds: u64,
    pub expires_at_unix_seconds: u64,
    pub consumed_at_unix_seconds: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ApprovalRequestRecord {
    pub approval_id: String,
    pub action_scope: String,
    pub target: String,
    pub requested_by: String,
    pub reason: String,
    pub risk: ActionRisk,
    pub status: ApprovalStatus,
    pub requested_at_unix_seconds: u64,
    pub operator_id: Option<String>,
    pub decision_reason: Option<String>,
    pub decided_at_unix_seconds: Option<u64>,
    pub issued_token: Option<ApprovalTokenRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ApprovalResolution {
    pub approval_id: String,
    pub status: ApprovalStatus,
    pub approval_token: Option<String>,
    pub expires_at_unix_seconds: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct ApprovalStoreFile {
    version: u32,
    approvals: Vec<ApprovalRequestRecord>,
}

pub fn request_approval(
    store_path: &Path,
    request: NewApprovalRequest,
) -> Result<ApprovalRequestRecord> {
    validate_new_approval_request(&request)?;
    let mut store = load_store(store_path)?;

    let record = ApprovalRequestRecord {
        approval_id: new_runtime_id("approval"),
        action_scope: request.action_scope,
        target: request.target,
        requested_by: request.requested_by,
        reason: request.reason,
        risk: request.risk,
        status: ApprovalStatus::Pending,
        requested_at_unix_seconds: current_unix_seconds(),
        operator_id: None,
        decision_reason: None,
        decided_at_unix_seconds: None,
        issued_token: None,
    };

    store.approvals.push(record.clone());
    save_store(store_path, &store)?;
    Ok(record)
}

pub fn resolve_approval(
    store_path: &Path,
    approval_id: &str,
    decision: ApprovalDecisionKind,
    operator_id: &str,
    reason: &str,
    expires_in_seconds: u64,
) -> Result<ApprovalResolution> {
    ensure!(
        !approval_id.trim().is_empty(),
        "approval id cannot be empty"
    );
    ensure!(
        !operator_id.trim().is_empty(),
        "operator id cannot be empty"
    );
    ensure!(!reason.trim().is_empty(), "decision reason cannot be empty");

    let mut store = load_store(store_path)?;
    let record = store
        .approvals
        .iter_mut()
        .find(|record| record.approval_id == approval_id)
        .ok_or_else(|| anyhow::anyhow!("approval '{}' was not found", approval_id))?;

    ensure!(
        record.status == ApprovalStatus::Pending,
        "approval '{}' has already been resolved",
        approval_id
    );

    record.operator_id = Some(operator_id.to_string());
    record.decision_reason = Some(reason.to_string());
    record.decided_at_unix_seconds = Some(current_unix_seconds());

    let resolution = match decision {
        ApprovalDecisionKind::Approve => {
            ensure!(
                expires_in_seconds > 0,
                "approved tokens must expire after at least one second"
            );
            let issued_at = current_unix_seconds();
            let token = ApprovalTokenRecord {
                token_id: new_runtime_id("approval-token"),
                approval_id: record.approval_id.clone(),
                action_scope: record.action_scope.clone(),
                target: record.target.clone(),
                issued_at_unix_seconds: issued_at,
                expires_at_unix_seconds: issued_at + expires_in_seconds,
                consumed_at_unix_seconds: None,
            };
            record.status = ApprovalStatus::Approved;
            record.issued_token = Some(token.clone());
            ApprovalResolution {
                approval_id: record.approval_id.clone(),
                status: record.status,
                approval_token: Some(token.token_id),
                expires_at_unix_seconds: Some(token.expires_at_unix_seconds),
            }
        }
        ApprovalDecisionKind::Deny => {
            record.status = ApprovalStatus::Denied;
            ApprovalResolution {
                approval_id: record.approval_id.clone(),
                status: record.status,
                approval_token: None,
                expires_at_unix_seconds: None,
            }
        }
    };

    save_store(store_path, &store)?;
    Ok(resolution)
}

pub fn consume_approval_token(
    store_path: &Path,
    token_id: &str,
    expected_action_scope: &str,
    expected_target: &str,
) -> Result<ApprovalTokenRecord> {
    ensure!(
        !token_id.trim().is_empty(),
        "approval token cannot be empty"
    );
    ensure!(
        !expected_action_scope.trim().is_empty(),
        "expected action scope cannot be empty"
    );
    ensure!(
        !expected_target.trim().is_empty(),
        "expected target cannot be empty"
    );

    let mut store = load_store(store_path)?;
    let now = current_unix_seconds();

    for approval in &mut store.approvals {
        if let Some(token) = &mut approval.issued_token {
            if token.token_id != token_id {
                continue;
            }

            ensure!(
                approval.status == ApprovalStatus::Approved,
                "approval token '{}' is not in approved state",
                token_id
            );
            ensure!(
                token.consumed_at_unix_seconds.is_none(),
                "approval token '{}' has already been consumed",
                token_id
            );
            ensure!(
                token.expires_at_unix_seconds >= now,
                "approval token '{}' has expired",
                token_id
            );
            ensure!(
                token.action_scope == expected_action_scope,
                "approval token '{}' does not match action scope '{}'",
                token_id,
                expected_action_scope
            );
            ensure!(
                token.target == expected_target,
                "approval token '{}' does not match target '{}'",
                token_id,
                expected_target
            );

            token.consumed_at_unix_seconds = Some(now);
            let consumed = token.clone();
            save_store(store_path, &store)?;
            return Ok(consumed);
        }
    }

    bail!("approval token '{}' was not found", token_id)
}

fn load_store(store_path: &Path) -> Result<ApprovalStoreFile> {
    let mut store: ApprovalStoreFile = read_json_or_default(store_path)?;
    if store.version == 0 {
        store.version = 1;
    }
    Ok(store)
}

fn save_store(store_path: &Path, store: &ApprovalStoreFile) -> Result<()> {
    write_pretty_json(store_path, store)
}

fn validate_new_approval_request(request: &NewApprovalRequest) -> Result<()> {
    ensure!(
        !request.action_scope.trim().is_empty(),
        "action scope cannot be empty"
    );
    ensure!(!request.target.trim().is_empty(), "target cannot be empty");
    ensure!(
        !request.requested_by.trim().is_empty(),
        "requested_by cannot be empty"
    );
    ensure!(!request.reason.trim().is_empty(), "reason cannot be empty");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn new_request() -> NewApprovalRequest {
        NewApprovalRequest {
            action_scope: "dataset.register".to_string(),
            target: "pdmx".to_string(),
            requested_by: "operator".to_string(),
            reason: "register approved dataset".to_string(),
            risk: ActionRisk::ApprovalRequired,
        }
    }

    #[test]
    fn test_request_approve_and_consume_token() {
        let dir = tempdir().unwrap();
        let store_path = dir.path().join("approvals.json");

        let request = request_approval(&store_path, new_request()).unwrap();
        let resolution = resolve_approval(
            &store_path,
            &request.approval_id,
            ApprovalDecisionKind::Approve,
            "approver",
            "looks good",
            300,
        )
        .unwrap();
        let token = consume_approval_token(
            &store_path,
            resolution.approval_token.as_deref().unwrap(),
            "dataset.register",
            "pdmx",
        )
        .unwrap();

        assert_eq!(resolution.status, ApprovalStatus::Approved);
        assert_eq!(token.target, "pdmx");
        assert!(token.consumed_at_unix_seconds.is_some());
    }

    #[test]
    fn test_denied_approval_has_no_token() {
        let dir = tempdir().unwrap();
        let store_path = dir.path().join("approvals.json");

        let request = request_approval(&store_path, new_request()).unwrap();
        let resolution = resolve_approval(
            &store_path,
            &request.approval_id,
            ApprovalDecisionKind::Deny,
            "approver",
            "not allowed",
            300,
        )
        .unwrap();

        assert_eq!(resolution.status, ApprovalStatus::Denied);
        assert!(resolution.approval_token.is_none());
    }

    #[test]
    fn test_consume_rejects_wrong_scope() {
        let dir = tempdir().unwrap();
        let store_path = dir.path().join("approvals.json");

        let request = request_approval(&store_path, new_request()).unwrap();
        let resolution = resolve_approval(
            &store_path,
            &request.approval_id,
            ApprovalDecisionKind::Approve,
            "approver",
            "looks good",
            300,
        )
        .unwrap();

        let error = consume_approval_token(
            &store_path,
            resolution.approval_token.as_deref().unwrap(),
            "publish.execute",
            "pdmx",
        )
        .unwrap_err();

        assert!(error.to_string().contains("action scope"));
    }

    #[test]
    fn test_consume_rejects_expired_token() {
        let dir = tempdir().unwrap();
        let store_path = dir.path().join("approvals.json");

        let request = request_approval(&store_path, new_request()).unwrap();
        let resolution = resolve_approval(
            &store_path,
            &request.approval_id,
            ApprovalDecisionKind::Approve,
            "approver",
            "looks good",
            300,
        )
        .unwrap();

        let mut store = load_store(&store_path).unwrap();
        let token = store.approvals[0].issued_token.as_mut().unwrap();
        token.expires_at_unix_seconds = 0;
        save_store(&store_path, &store).unwrap();

        let error = consume_approval_token(
            &store_path,
            resolution.approval_token.as_deref().unwrap(),
            "dataset.register",
            "pdmx",
        )
        .unwrap_err();

        assert!(error.to_string().contains("expired"));
    }
}
