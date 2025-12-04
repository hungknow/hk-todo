#[cfg(feature = "dart")]
use flutter_rust_bridge::frb;

/// Value object representing the state of a Todo in its workflow
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "dart", frb)]
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

    /// Validates if a transition to the new state is allowed
    /// 
    /// # Parameters
    /// - `self`: Current TodoState
    /// - `new_state`: Target state to transition to
    /// 
    /// # Returns
    /// - `bool`: `false` if same state or invalid transition, `true` if transition allowed
    /// 
    /// # Special Requirements
    /// - Uses `can_advance()` for forward transitions
    /// - Uses `can_retreat()` for backward transitions
    /// - Validates workflow: Todo → InProgress → Done (and backwards)
    pub fn can_transition_to(&self, new_state: TodoState) -> bool {
        if *self == new_state {
            return false;
        }

        // Determine if transition is forward or backward
        let is_forward = match (*self, new_state) {
            (TodoState::Todo, TodoState::InProgress) => true,
            (TodoState::InProgress, TodoState::Done) => true,
            (TodoState::Done, TodoState::InProgress) => false,
            (TodoState::InProgress, TodoState::Todo) => false,
            // Invalid transitions (skipping states or same state - already checked above)
            _ => return false,
        };

        // Validate using TodoState methods
        if is_forward {
            self.can_advance()
        } else {
            self.can_retreat()
        }
    }
}

