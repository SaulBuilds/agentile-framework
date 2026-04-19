use std::collections::{HashMap, VecDeque};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Represents a state in a state machine
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct State {
    /// Unique identifier for this state
    pub id: String,
    /// Optional data associated with this state
    pub data: Option<serde_json::Value>,
}

impl State {
    /// Create a new state
    pub fn new<S: Into<String>>(id: S, data: Option<serde_json::Value>) -> Self {
        State {
            id: id.into(),
            data,
        }
    }
}

/// Represents a transition between states in a state machine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transition {
    /// Source state ID
    pub from: String,
    /// Target state ID
    pub to: String,
    /// Optional condition that must be true for this transition to occur
    pub condition: Option<String>,
    /// Optional action to perform when this transition occurs
    pub action: Option<String>,
    /// Priority of this transition (higher numbers = higher priority)
    pub priority: u32,
}

impl Transition {
    /// Create a new transition
    pub fn new<S: Into<String>>(
        from: S,
        to: S,
        condition: Option<String>,
        action: Option<String>,
        priority: u32,
    ) -> Self {
        Transition {
            from: from.into(),
            to: to.into(),
            condition,
            action,
            priority,
        }
    }
}

/// Errors that can occur when working with state machines
#[derive(Debug, Error)]
pub enum StateMachineError {
    #[error("State not found: {0}")]
    StateNotFound(String),
    
    #[error("Initial state not set")]
    InitialStateNotSet,
    
    #[error("No valid transition from state '{from}' with event '{event}'")]
    NoValidTransition { from: String, event: String },
    
    #[error("Transition condition evaluated to false")]
    ConditionFalse,
    
    #[error("Other error: {0}")]
    Other(#[from] anyhow::Error),
}

/// A finite state machine that can be controlled by events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateMachine {
    /// Current state of the machine
    #[serde(skip)]
    current_state: Option<String>,
    /// All states in the machine
    states: HashMap<String, State>,
    /// All transitions in the machine
    transitions: Vec<Transition>,
    /// Event queue for asynchronous processing
    event_queue: VecDeque<String>,
    /// Whether the machine is currently running
    running: bool,
}

impl StateMachine {
    /// Create a new state machine
    pub fn new() -> Self {
        StateMachine {
            current_state: None,
            states: HashMap::new(),
            transitions: Vec::new(),
            event_queue: VecDeque::new(),
            running: false,
        }
    }

    /// Add a state to the machine
    pub fn add_state(&mut self, state: State) -> &mut Self {
        self.states.insert(state.id.clone(), state);
        self
    }

    /// Add a transition to the machine
    pub fn add_transition(&mut self, transition: Transition) -> &mut Self {
        // Insert the transition in priority order (higher priority first)
        let mut inserted = false;
        for i in 0..self.transitions.len() {
            if transition.priority > self.transitions[i].priority {
                self.transitions.insert(i, transition.clone());
                inserted = true;
                break;
            }
        }
        if !inserted {
            self.transitions.push(transition);
        }
        self
    }

    /// Set the initial state of the machine
    pub fn set_initial_state<S: Into<String>>(&mut self, state_id: S) -> Result<(), StateMachineError> {
        let state_id = state_id.into();
        if !self.states.contains_key(&state_id) {
            return Err(StateMachineError::StateNotFound(state_id));
        }
        self.current_state = Some(state_id);
        Ok(())
    }

    /// Get the current state ID
    pub fn current_state_id(&self) -> Option<&String> {
        self.current_state.as_ref()
    }

    /// Get the current state
    pub fn current_state(&self) -> Option<&State> {
        self.current_state.as_ref().and_then(|id| self.states.get(id))
    }

    /// Queue an event for processing
    pub fn queue_event<S: Into<String>>(&mut self, event: S) {
        self.event_queue.push_back(event.into());
    }

    /// Process the next event in the queue
    pub fn process_next_event(&mut self) -> Result<Option<String>, StateMachineError> {
        if !self.running {
            return Ok(None);
        }

        let event = self.event_queue.pop_front();
        if let Some(event) = event {
            self.transition(&event)?;
            Ok(Some(event))
        } else {
            Ok(None)
        }
    }

    /// Process all events in the queue
    pub fn process_all_events(&mut self) -> Result<Vec<String>, StateMachineError> {
        if !self.running {
            return Ok(Vec::new());
        }

        let mut processed = Vec::new();
        while let Some(event) = self.event_queue.pop_front() {
            self.transition(&event)?;
            processed.push(event);
        }
        Ok(processed)
    }

    /// Start the state machine
    pub fn start(&mut self) -> Result<(), StateMachineError> {
        if self.current_state.is_none() {
            return Err(StateMachineError::InitialStateNotSet);
        }
        self.running = true;
        Ok(())
    }

    /// Stop the state machine
    pub fn stop(&mut self) {
        self.running = false;
        self.event_queue.clear();
    }

    /// Check if the machine is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Perform a state transition based on an event
    fn transition(&mut self, event: &str) -> Result<(), StateMachineError> {
        let current_state_id = self.current_state.as_ref()
            .ok_or(StateMachineError::InitialStateNotSet)?
            .clone();

        // Find all valid transitions from the current state with this event
        let mut valid_transitions: Vec<&Transition> = self.transitions
            .iter()
            .filter(|t| t.from == current_state_id)
            .filter(|t| {
                // Check if condition is met (if specified)
                if let Some(condition) = &t.condition {
                    // Condition is met if it matches the event
                    condition == event
                } else {
                    true
                }
            })
            .collect();

        // Sort by: 1) transitions with conditions first (true > false), 2) then by priority (higher first)
        valid_transitions.sort_by(|a, b| {
            let a_has_condition = a.condition.is_some();
            let b_has_condition = b.condition.is_some();
            if a_has_condition != b_has_condition {
                // Transitions with conditions come first
                b_has_condition.cmp(&a_has_condition)
            } else {
                // Same condition status, sort by priority
                b.priority.cmp(&a.priority)
            }
        });

        // Take the highest priority valid transition
        if let Some(transition) = valid_transitions.first() {
            // Execute action if specified
            if let Some(action) = &transition.action {
                // In a real implementation, we would execute the action
                tracing::debug!("Executing action: {}", action);
            }

            // Change state
            self.current_state = Some(transition.to.clone());
            tracing::debug!("Transitioned from {} to {} via event {}", current_state_id, transition.to, event);
            Ok(())
        } else {
            Err(StateMachineError::NoValidTransition {
                from: current_state_id,
                event: event.to_string(),
            })
        }
    }
}

impl Default for StateMachine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_state_machine_creation() {
        let sm = StateMachine::new();
        assert!(sm.current_state.is_none());
        assert!(sm.states.is_empty());
        assert!(sm.transitions.is_empty());
        assert!(!sm.is_running());
    }

    #[test]
    fn test_add_state() {
        let mut sm = StateMachine::new();
        let state = State::new("state1", Some(json!({"value": 42})));
        sm.add_state(state);
        
        assert_eq!(sm.states.len(), 1);
        assert!(sm.states.contains_key("state1"));
        let state = sm.states.get("state1").unwrap();
        assert_eq!(state.id, "state1");
        assert_eq!(state.data, Some(json!({"value": 42})));
    }

    #[test]
    fn test_add_transition() {
        let mut sm = StateMachine::new();
        sm.add_state(State::new("state1", None));
        sm.add_state(State::new("state2", None));
        
        let transition = Transition::new("state1", "state2", Some("condition".to_string()), Some("action".to_string()), 10);
        sm.add_transition(transition);
        
        assert_eq!(sm.transitions.len(), 1);
        assert_eq!(sm.transitions[0].from, "state1");
        assert_eq!(sm.transitions[0].to, "state2");
        assert_eq!(sm.transitions[0].condition, Some("condition".to_string()));
        assert_eq!(sm.transitions[0].action, Some("action".to_string()));
        assert_eq!(sm.transitions[0].priority, 10);
    }

    #[test]
    fn test_set_initial_state() {
        let mut sm = StateMachine::new();
        sm.add_state(State::new("state1", None));
        sm.add_state(State::new("state2", None));
        
        assert!(sm.set_initial_state("state1").is_ok());
        assert_eq!(sm.current_state_id(), Some(&"state1".to_string()));
        
        assert!(sm.set_initial_state("nonexistent").is_err());
    }

    #[test]
    fn test_transition() {
        let mut sm = StateMachine::new();
        sm.add_state(State::new("state1", None));
        sm.add_state(State::new("state2", None));
        sm.add_state(State::new("state3", None));
        
        // Add transitions
        sm.add_transition(Transition::new("state1", "state2", None, None, 10));
        sm.add_transition(Transition::new("state1", "state3", Some("go_to_state3".to_string()), None, 5));
        
        sm.set_initial_state("state1").unwrap();
        sm.start().unwrap();
        
        // Process an event that triggers the first transition (no condition, higher priority)
        sm.queue_event("go_to_state2");
        assert!(sm.process_next_event().unwrap().is_some());
        assert_eq!(sm.current_state_id(), Some(&"state2".to_string()));
        
        // Reset and test conditional transition
        sm.set_initial_state("state1").unwrap();
        sm.queue_event("go_to_state3");
        assert!(sm.process_next_event().unwrap().is_some());
        assert_eq!(sm.current_state_id(), Some(&"state3".to_string()));
    }

    #[test]
    fn test_no_valid_transition() {
        let mut sm = StateMachine::new();
        sm.add_state(State::new("state1", None));
        sm.add_state(State::new("state2", None));
        
        sm.set_initial_state("state1").unwrap();
        sm.start().unwrap();
        
        // Queue an event with no matching transition
        sm.queue_event("nonexistent_event");
        assert!(sm.process_next_event().is_err());
    }
}