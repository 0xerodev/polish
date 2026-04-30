pub struct StateMachineHarness<S: Clone + Eq + std::fmt::Debug, E: Clone + std::fmt::Debug> {
    current: S,
    history: Vec<(S, E, S)>,
    transitions: Vec<(S, E, S)>,
}

impl<S: Clone + Eq + std::fmt::Debug, E: Clone + Eq + std::fmt::Debug> StateMachineHarness<S, E> {
    pub fn new(initial: S) -> Self {
        Self { current: initial, history: Vec::new(), transitions: Vec::new() }
    }

    pub fn allow(mut self, from: S, event: E, to: S) -> Self {
        self.transitions.push((from, event, to));
        self
    }

    pub fn send(&mut self, event: E) -> Result<&S, String> {
        let from = self.current.clone();
        let transition = self.transitions.iter().find(|(s, e, _)| *s == from && *e == event);
        match transition {
            Some((_, _, to)) => {
                let to = to.clone();
                self.history.push((from, event, to.clone()));
                self.current = to;
                Ok(&self.current)
            }
            None => Err(format!("Invalid transition from {from:?} on {event:?}")),
        }
    }

    pub fn state(&self) -> &S {
        &self.current
    }

    pub fn history_len(&self) -> usize {
        self.history.len()
    }

    pub fn assert_state(&self, expected: &S) -> Result<(), String> {
        if &self.current == expected {
            Ok(())
        } else {
            Err(format!("Expected state {expected:?}, got {:?}", self.current))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Eq, PartialEq, Debug)]
    enum State { Idle, Active, Done }
    #[derive(Clone, Eq, PartialEq, Debug)]
    enum Event { Start, Complete }

    #[test]
    fn state_machine_transitions() {
        let mut m = StateMachineHarness::new(State::Idle)
            .allow(State::Idle, Event::Start, State::Active)
            .allow(State::Active, Event::Complete, State::Done);

        m.send(Event::Start).unwrap();
        m.assert_state(&State::Active).unwrap();
        m.send(Event::Complete).unwrap();
        m.assert_state(&State::Done).unwrap();
    }

    #[test]
    fn invalid_transition_rejected() {
        let mut m = StateMachineHarness::new(State::Idle)
            .allow(State::Idle, Event::Start, State::Active);
        let r = m.send(Event::Complete);
        assert!(r.is_err());
    }
}
