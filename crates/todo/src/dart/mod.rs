// Re-export domain types for code generation
// These are type-only exports - no impl blocks to avoid opaque/non-opaque duplication
pub use crate::domain::todo::{
    Todo, TodoError, TodoEvent, TodoRepository, TodoState,
};
