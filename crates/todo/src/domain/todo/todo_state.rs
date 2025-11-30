/// Value object representing the state of a Todo in its workflow
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TodoState {
    /// Initial state when a todo is created
    Todo,
    /// Intermediate state indicating work in progress
    InProgress,
    /// Final state indicating completion
    Done,
}

impl TodoState {
    /// Checks if the state can advance to the next state
    /// 
    /// # Parameters
    /// - `self`: Reference to TodoState
    /// 
    /// # Returns
    /// - `bool`: `true` for `Todo`/`InProgress`, `false` for `Done`
    pub fn can_advance(&self) -> bool {
        matches!(self, TodoState::Todo | TodoState::InProgress)
    }

    /// Checks if the state can retreat to the previous state
    /// 
    /// # Parameters
    /// - `self`: Reference to TodoState
    /// 
    /// # Returns
    /// - `bool`: `false` for `Todo`, `true` for `InProgress`/`Done`
    pub fn can_retreat(&self) -> bool {
        matches!(self, TodoState::InProgress | TodoState::Done)
    }
}

