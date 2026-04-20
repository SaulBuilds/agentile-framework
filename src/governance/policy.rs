use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Orchestration policy governs execution boundaries for harness plans
/// and scheduled job runs.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct OrchestrationPolicy {
    /// Maximum number of actions a single harness plan may propose.
    pub max_actions_per_plan: usize,
    /// Maximum number of realtime dispatches a single scheduled job run may perform.
    pub max_dispatches_per_job_run: usize,
    /// When true, the executor is running inside a scheduled job and must not
    /// call `schedule_job()` or similar job-creation functions.
    pub in_scheduled_job_context: bool,
}

impl Default for OrchestrationPolicy {
    fn default() -> Self {
        Self {
            max_actions_per_plan: 10,
            max_dispatches_per_job_run: 20,
            in_scheduled_job_context: false,
        }
    }
}

impl OrchestrationPolicy {
    pub fn for_scheduled_job() -> Self {
        Self {
            in_scheduled_job_context: true,
            ..Self::default()
        }
    }

    pub fn validate_plan_action_count(&self, action_count: usize) -> Result<(), PolicyViolation> {
        if action_count > self.max_actions_per_plan {
            Err(PolicyViolation::TooManyActions {
                requested: action_count,
                limit: self.max_actions_per_plan,
            })
        } else {
            Ok(())
        }
    }

    pub fn validate_dispatch_count(&self, dispatch_count: usize) -> Result<(), PolicyViolation> {
        if dispatch_count > self.max_dispatches_per_job_run {
            Err(PolicyViolation::TooManyDispatches {
                requested: dispatch_count,
                limit: self.max_dispatches_per_job_run,
            })
        } else {
            Ok(())
        }
    }

    pub fn validate_no_recursive_scheduling(&self) -> Result<(), PolicyViolation> {
        if self.in_scheduled_job_context {
            Err(PolicyViolation::RecursiveScheduling)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyViolation {
    TooManyActions { requested: usize, limit: usize },
    TooManyDispatches { requested: usize, limit: usize },
    RecursiveScheduling,
}

impl std::fmt::Display for PolicyViolation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PolicyViolation::TooManyActions { requested, limit } => {
                write!(
                    f,
                    "plan proposes {requested} actions but policy allows at most {limit}"
                )
            }
            PolicyViolation::TooManyDispatches { requested, limit } => {
                write!(
                    f,
                    "job run attempted {requested} dispatches but policy allows at most {limit}"
                )
            }
            PolicyViolation::RecursiveScheduling => {
                write!(
                    f,
                    "scheduling new jobs from inside a scheduled job run is not allowed"
                )
            }
        }
    }
}

impl std::error::Error for PolicyViolation {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_policy_accepts_reasonable_counts() {
        let policy = OrchestrationPolicy::default();
        assert!(policy.validate_plan_action_count(5).is_ok());
        assert!(policy.validate_dispatch_count(10).is_ok());
        assert!(policy.validate_no_recursive_scheduling().is_ok());
    }

    #[test]
    fn test_policy_rejects_excessive_actions() {
        let policy = OrchestrationPolicy {
            max_actions_per_plan: 3,
            ..Default::default()
        };
        assert!(policy.validate_plan_action_count(3).is_ok());
        let err = policy.validate_plan_action_count(4).unwrap_err();
        assert!(matches!(
            err,
            PolicyViolation::TooManyActions {
                requested: 4,
                limit: 3
            }
        ));
    }

    #[test]
    fn test_policy_rejects_excessive_dispatches() {
        let policy = OrchestrationPolicy {
            max_dispatches_per_job_run: 2,
            ..Default::default()
        };
        assert!(policy.validate_dispatch_count(2).is_ok());
        let err = policy.validate_dispatch_count(3).unwrap_err();
        assert!(matches!(
            err,
            PolicyViolation::TooManyDispatches {
                requested: 3,
                limit: 2
            }
        ));
    }

    #[test]
    fn test_scheduled_job_context_blocks_recursive_scheduling() {
        let policy = OrchestrationPolicy::for_scheduled_job();
        let err = policy.validate_no_recursive_scheduling().unwrap_err();
        assert!(matches!(err, PolicyViolation::RecursiveScheduling));
    }
}
