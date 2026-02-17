#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppState {
    Testing,
    Results,
    History,
    Stats,
}

pub struct StateMachine {
    current: AppState,
    previous: Option<AppState>,
}

impl StateMachine {
    pub fn new(initial: AppState) -> Self {
        Self {
            current: initial,
            previous: None,
        }
    }

    pub fn current(&self) -> AppState {
        self.current
    }

    pub fn transition(&mut self, new_state: AppState) {
        self.previous = Some(self.current);
        self.current = new_state;
    }
}
