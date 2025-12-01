pub mod domain;
pub mod infrastructure;
pub mod application;

#[cfg(feature = "python")]
pub mod python;

#[cfg(feature = "dart")]
pub mod dart;

// Re-export commonly used domain types for convenience
pub use domain::todo::{
    Todo, TodoError, TodoEvent, TodoRepository, TodoState,
};

