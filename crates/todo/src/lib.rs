pub mod domain;
pub mod infrastructure;
pub mod application;

// Re-export commonly used domain types for convenience
pub use domain::todo::{
    Todo, TodoError, TodoEvent, TodoRepository, TodoState,
};

