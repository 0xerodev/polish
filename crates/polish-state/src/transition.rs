use crate::machine::{State, Event};

pub type TransitionGuard = Box<dyn Fn() -> bool + Send + Sync>;

#[derive(Clone)]
pub struct Transition {
    pub from: State,
    pub event: Event,
    pub to: State,
    pub guard: Option<std::sync::Arc<dyn Fn() -> bool + Send + Sync>>,
}

impl std::fmt::Debug for Transition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} --[{}]--> {}", self.from, self.event.0, self.to)
    }
}
