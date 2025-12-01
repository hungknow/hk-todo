use flutter_rust_bridge::frb;
use crate::{Todo, TodoState, TodoError, TodoEvent};

// Re-export types that will be used in Dart
// These types need to be marked with #[frb] attributes for code generation

/// Create a new Todo instance
#[frb(sync)]
pub fn create_todo(description: String) -> Result<Todo, TodoError> {
    let (todo, _events) = Todo::new(description)?;
    Ok(todo)
}

/// Create a new Todo instance and return the created events
#[frb(sync)]
pub fn create_todo_with_events(description: String) -> Result<(Todo, Vec<TodoEvent>), TodoError> {
    Todo::new(description)
}

/// Change todo to the next state in the workflow
#[frb(sync)]
pub fn todo_change_to_next_state(todo: &mut Todo) -> Result<Vec<TodoEvent>, TodoError> {
    todo.change_to_next_state()
}

/// Change todo to the previous state in the workflow
#[frb(sync)]
pub fn todo_change_to_previous_state(todo: &mut Todo) -> Result<Vec<TodoEvent>, TodoError> {
    todo.change_to_previous_state()
}

/// Update todo to a specific state
#[frb(sync)]
pub fn todo_update_state(todo: &mut Todo, new_state: TodoState) -> Result<Vec<TodoEvent>, TodoError> {
    todo.update_state(new_state)
}

