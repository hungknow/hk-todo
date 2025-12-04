#[cfg(feature = "dart")]
use flutter_rust_bridge::frb;

/// Error types for Todo domain operations
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dart", frb)]
pub enum TodoError {
    /// Returned when attempting to create a Todo with an empty description
    EmptyDescription,
    /// Returned when attempting an invalid state transition
    InvalidStateTransition,
    /// Returned when a Todo is not found in the repository
    TodoNotFound,
}

