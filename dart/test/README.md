# Dart Integration Tests

This directory contains integration tests for the Todo domain using Dart language bindings.

## Test Files

- `add_todo_handler_test.dart` - Tests for AddTodoHandler functionality (create_todo, create_todo_with_events)
- `change_todo_state_handler_test.dart` - Tests for ChangeTodoStateHandler functionality (todo_update_state, todo_change_to_next_state, todo_change_to_previous_state)
- `get_todos_handler_test.dart` - Tests for GetTodosHandler functionality (domain functions that would be used by the handler)
- `test_helper.dart` - Common test utilities and helpers

## Setup

Before running these tests, ensure:

1. The Rust library is built:
   ```bash
   cargo build
   ```

2. The Dart bindings are generated:
   ```bash
   cd crates/todo
   flutter_rust_bridge_codegen generate
   ```

3. Dart dependencies are installed:
   ```bash
   cd dart
   dart pub get
   ```

## Running Tests

```bash
cd dart
dart test
```

## Note on API Structure

These tests are written based on the expected API structure from the Rust code in `crates/todo/src/dart/mod.rs`. The actual generated API may have slightly different naming conventions (e.g., snake_case vs camelCase). If the tests fail due to API mismatches, adjust the function calls to match the actual generated bindings.

The expected API functions are:
- `create_todo(description: String) -> Result<Todo, TodoError>`
- `create_todo_with_events(description: String) -> Result<(Todo, Vec<TodoEvent>), TodoError>`
- `todo_change_to_next_state(todo: &mut Todo) -> Result<Vec<TodoEvent>, TodoError>`
- `todo_change_to_previous_state(todo: &mut Todo) -> Result<Vec<TodoEvent>, TodoError>`
- `todo_update_state(todo: &mut Todo, new_state: TodoState) -> Result<Vec<TodoEvent>, TodoError>`

