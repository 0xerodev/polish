
use crate::transition::Transition;
use crate::error::StateError;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct State(pub String);

impl State {
    pub fn new(s: impl Into<String>) -> Self { Self(s.into()) }
    pub fn as_str(&self) -> &str { &self.0 }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Event(pub String);

impl Event {
    pub fn new(s: impl Into<String>) -> Self { Self(s.into()) }
}

/// A typed state machine. Defines valid state transitions.
#[derive(Clone, Debug)]
pub struct StateMachine {
    pub name: String,
    pub initial: State,
    current: State,
    transitions: Vec<Transition>,
    history: Vec<(State, Event, State)>,
}

impl StateMachine {
    pub fn new(name: impl Into<String>, initial: State) -> Self {
        let s = initial.clone();
        Self {
            name: name.into(),
            initial,
            current: s,
            transitions: Vec::new(),
            history: Vec::new(),
        }
    }

    pub fn add_transition(mut self, t: Transition) -> Self {
        self.transitions.push(t); self
    }

    pub fn current(&self) -> &State { &self.current }

    pub fn is_in(&self, state: &State) -> bool { &self.current == state }

    pub fn can_transition(&self, event: &Event) -> bool {
        self.transitions.iter().any(|t| t.from == self.current && t.event == *event)
    }

    pub fn transition(&mut self, event: &Event) -> Result<&State, StateError> {
        let t = self.transitions.iter()
            .find(|t| t.from == self.current && t.event == *event)
            .cloned()
            .ok_or_else(|| StateError::InvalidTransition {
                from: self.current.0.clone(),
                event: event.0.clone(),
            })?;

        // Check guard if present
        if let Some(guard) = &t.guard {
            if !guard() {
                return Err(StateError::GuardFailed { state: self.current.0.clone() });
            }
        }

        let prev = self.current.clone();
        self.current = t.to.clone();
        self.history.push((prev, event.clone(), t.to.clone()));
        Ok(&self.current)
    }

    pub fn reset(&mut self) {
        self.current = self.initial.clone();
        self.history.clear();
    }

    pub fn history(&self) -> &[(State, Event, State)] { &self.history }

    /// All valid events from the current state.
    pub fn valid_events(&self) -> Vec<&Event> {
        self.transitions.iter()
            .filter(|t| t.from == self.current)
            .map(|t| &t.event)
            .collect()
    }

    /// Validate no impossible states exist in the transition table.
    pub fn validate(&self) -> Vec<String> {
        let mut issues = Vec::new();
        let all_states: std::collections::HashSet<_> = self.transitions.iter()
            .flat_map(|t| [&t.from, &t.to])
            .collect();
        // Check initial is reachable
        if !all_states.contains(&self.initial) && !self.transitions.is_empty() {
            issues.push(format!("Initial state '{}' never appears in transitions", self.initial));
        }
        issues
    }
}

/// Fluent builder for state machines.
pub struct MachineBuilder {
    name: String,
    initial: Option<State>,
    transitions: Vec<Transition>,
}

impl MachineBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), initial: None, transitions: Vec::new() }
    }

    pub fn initial(mut self, s: impl Into<String>) -> Self {
        self.initial = Some(State::new(s)); self
    }

    pub fn on(mut self, from: impl Into<String>, event: impl Into<String>, to: impl Into<String>) -> Self {
        self.transitions.push(Transition {
            from: State::new(from), event: Event::new(event),
            to: State::new(to), guard: None,
        });
        self
    }

    pub fn build(self) -> StateMachine {
        let initial = self.initial.expect("initial state required");
        let mut m = StateMachine::new(self.name, initial);
        for t in self.transitions { m = m.add_transition(t); }
        m
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn transfer_machine() -> StateMachine {
        MachineBuilder::new("transfer")
            .initial("idle")
            .on("idle",      "valid_input", "ready")
            .on("ready",     "submit",      "executing")
            .on("executing", "confirmed",   "success")
            .on("executing", "failed",      "error")
            .on("error",     "valid_input", "ready")
            .on("success",   "new_intent",  "idle")
            .build()
    }

    #[test]
    fn happy_path() {
        let mut m = transfer_machine();
        assert_eq!(m.current().as_str(), "idle");
        m.transition(&Event::new("valid_input")).unwrap();
        m.transition(&Event::new("submit")).unwrap();
        m.transition(&Event::new("confirmed")).unwrap();
        assert_eq!(m.current().as_str(), "success");
    }

    #[test]
    fn invalid_transition_rejected() {
        let mut m = transfer_machine();
        let err = m.transition(&Event::new("submit"));
        assert!(err.is_err());
    }

    #[test]
    fn error_recovery() {
        let mut m = transfer_machine();
        m.transition(&Event::new("valid_input")).unwrap();
        m.transition(&Event::new("submit")).unwrap();
        m.transition(&Event::new("failed")).unwrap();
        assert_eq!(m.current().as_str(), "error");
        m.transition(&Event::new("valid_input")).unwrap();
        assert_eq!(m.current().as_str(), "ready");
    }
}
