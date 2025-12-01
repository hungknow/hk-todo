# Todo Dart Rust Bridge Integration Guide

This document describes the file tree structure for the Todo Rust library and its Dart bindings, and explains how to connect them using flutter_rust_bridge.

## Overview

The Todo project uses flutter_rust_bridge to create Dart bindings for the Rust library, allowing Dart developers to use the high-performance Rust implementation while maintaining a natural Dart API. flutter_rust_bridge automatically generates all the necessary FFI glue code between Rust and Dart. This integration uses pure Dart (no Flutter dependency), making it suitable for Dart CLI applications, servers, and other non-Flutter use cases.

**Key Features:**
- **Automatic code generation**: Write normal Rust code, and flutter_rust_bridge generates Dart bindings
- **Type safety**: Automatic translation of Rust types to Dart types
- **Async support**: Full support for async Rust functions
- **Cross-platform**: Works on Windows, Linux, macOS (and can be used in Flutter apps for mobile/Web if needed)

## File Tree Structure

### Rust Library Structure

```
crates/todo/
├── Cargo.toml                    # Rust package configuration
├── src/
│   ├── lib.rs                    # Library root, re-exports domain types
│   ├── domain/
│   │   ├── mod.rs
│   │   └── todo/
│   │       ├── mod.rs            # Domain module exports
│   │       ├── todo_entity.rs    # Todo aggregate root
│   │       ├── todo_state.rs     # TodoState enum (Todo, InProgress, Done)
│   │       ├── todo_error.rs     # TodoError enum
│   │       ├── todo_event.rs     # TodoEvent enum
│   │       └── todo_repository.rs # TodoRepository trait
│   ├── application/
│   │   ├── mod.rs
│   │   ├── add_todo_handler.rs   # Application handler for creating todos
│   │   ├── get_todos_handler.rs  # Application handler for retrieving todos
│   │   └── change_todo_state_handler.rs # Application handler for state changes
│   ├── infrastructure/
│   │   ├── mod.rs
│   │   └── repositories/
│   │       └── todo/
│   │           ├── mod.rs
│   │           └── inmemory_todo_repository.rs # In-memory repository implementation
│   └── dart/                     # Flutter Rust Bridge bindings module
│       └── mod.rs                # Dart bindings API definitions
└── tests/
    ├── add_todo.rs
    ├── change_todo_state.rs
    └── common/
        ├── mod.rs
        └── mock_todo_repository.rs
```

### Dart Bindings Structure

```
crates/dart-todo/
├── Cargo.toml                    # Root crate configuration (depends on todo crate)
├── src/
│   └── lib.rs                    # Re-exports Dart bindings from todo::dart
└── flutter_rust_bridge.yaml     # Code generation configuration
```

### Dart Package Structure

```
dart/
├── pubspec.yaml                  # Dart package dependencies (includes flutter_rust_bridge)
├── lib/
│   ├── todo.dart                 # Dart package entry point
│   ├── bindings/
│   │   └── todo_bridge.dart      # Auto-generated Dart bindings (do not edit)
│   └── src/
│       └── todo_client.dart      # Dart client wrapper (optional)
├── test/
│   └── todo_test.dart            # Dart tests
└── rust/                          # Rust source code (symlink or copy from crates/dart-todo)
    └── src/
        └── api.rs                # Flutter Rust Bridge API definitions
```

### Workspace Structure

```
todo-ddd/
├── Cargo.toml                    # Workspace configuration
├── crates/
│   ├── todo/                     # Core Rust library (domain logic)
│   └── dart-todo/                # Dart bindings crate (depends on todo)
├── dart/                          # Dart package
└── docs/
    ├── todo-domain-model.md
    ├── todo_pyo3.md
    └── todo_dart.md              # This file
```

## Tools Required

### 1. flutter_rust_bridge_codegen

The code generator that creates Dart bindings from Rust code.

**Installation:**
```bash
cargo install flutter_rust_bridge_codegen
```

**Usage:**
```bash
flutter_rust_bridge_codegen generate
```

### 2. flutter_rust_bridge (Dart Package)

The runtime library that handles FFI communication between Dart and Rust.

**Add to `pubspec.yaml`:**
```yaml
dependencies:
  flutter_rust_bridge: ^2.11.1
  ffi: ^2.1.4
  path: ^1.9.0
```

### 3. Dart SDK

Ensure Dart SDK is installed and configured:
```bash
dart --version
```

### 4. Rust Toolchain

Ensure Rust is installed:
```bash
rustc --version
cargo --version
```

## Setup Steps

### Step 1: Add flutter_rust_bridge Dependency to Rust

Update `crates/todo/Cargo.toml` to include flutter_rust_bridge:

```toml
[package]
name = "todo"
version = "0.1.0"
edition = "2024"

[lib]
name = "todo"
crate-type = ["rlib", "staticlib", "cdylib"]

[dependencies]
chrono = { workspace = true }
async-trait = { workspace = true }
uuid = { workspace = true }
flutter_rust_bridge = "2.0.0"

[features]
default = []
dart = ["flutter_rust_bridge/dart"]
```

### Step 2: Create Dart Bindings Module

Create `crates/todo/src/dart/mod.rs` with the API definitions:

```rust
use flutter_rust_bridge::frb;
use crate::{Todo, TodoState, TodoError, TodoEvent};

// Re-export types that will be used in Dart
#[frb(sync)]
pub fn create_todo(description: String) -> Result<Todo, TodoError> {
    let (todo, _events) = Todo::new(description)?;
    Ok(todo)
}

#[frb(sync)]
pub fn create_todo_with_events(description: String) -> Result<(Todo, Vec<TodoEvent>), TodoError> {
    Todo::new(description)
}

#[frb(sync)]
pub fn todo_change_to_next_state(todo: &mut Todo) -> Result<Vec<TodoEvent>, TodoError> {
    todo.change_to_next_state()
}

#[frb(sync)]
pub fn todo_change_to_previous_state(todo: &mut Todo) -> Result<Vec<TodoEvent>, TodoError> {
    todo.change_to_previous_state()
}

#[frb(sync)]
pub fn todo_update_state(todo: &mut Todo, new_state: TodoState) -> Result<Vec<TodoEvent>, TodoError> {
    todo.update_state(new_state)
}
```

Update `crates/todo/src/lib.rs` to conditionally expose the dart module:

```rust
pub mod domain;
pub mod infrastructure;
pub mod application;

#[cfg(feature = "dart")]
pub mod dart;

// Re-export commonly used domain types for convenience
pub use domain::todo::{
    Todo, TodoError, TodoEvent, TodoRepository, TodoState,
};
```

### Step 3: Create dart-todo Root Crate

Create `crates/dart-todo/` as a root crate:

```
crates/dart-todo/
├── Cargo.toml
└── src/
    └── lib.rs
```

### Step 4: Configure Cargo.toml for dart-todo

The `crates/dart-todo/Cargo.toml` should depend on the `todo` crate with the `dart` feature:

```toml
[package]
name = "dart_todo"
version = "0.1.0"
edition = "2024"

[lib]
name = "dart_todo"
crate-type = ["cdylib", "rlib"]

[dependencies]
todo = { path = "../todo", features = ["dart"] }
flutter_rust_bridge = "2.0.0"
```

### Step 5: Create Flutter Rust Bridge Configuration

Create `crates/dart-todo/flutter_rust_bridge.yaml`:

```yaml
# Flutter Rust Bridge code generation configuration

# Rust input
rust_input: src/lib.rs

# Dart output
dart_output: ../../dart/lib/bindings/todo_bridge.dart

# Additional options
[options]
# Enable async support
async: true
# Codec to use (sse is faster for some workloads)
codec: sse
```

### Step 6: Generate Dart Bindings

Run the code generator:

```bash
cd crates/dart-todo
flutter_rust_bridge_codegen generate
```

This will generate `dart/lib/bindings/todo_bridge.dart` with all the necessary Dart bindings.

### Step 7: Configure Dart Package

Update `dart/pubspec.yaml`:

```yaml
name: todo_dart
description: Todo Dart package with Rust backend

environment:
  sdk: ^3.9.2

# Add regular dependencies here.
dependencies:
  path: ^1.9.0
  flutter_rust_bridge: ^2.11.1
  ffi: ^2.1.4

dev_dependencies:
  lints: ^6.0.0
  test: ^1.25.6
  ffigen: ^20.1.0
```

### Step 8: Initialize Rust in Dart

Create `dart/lib/todo.dart` to initialize the Rust library:

```dart
import 'dart:io';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:path/path.dart' as path;
import 'bindings/todo_bridge.dart';

/// Initialize the Rust library
Future<void> initRustLib() async {
  // Determine the library path based on the platform
  final libraryPath = _getLibraryPath();
  final externalLibrary = ExternalLibrary.open(libraryPath);
  
  await RustLib.init(externalLibrary: externalLibrary);
}

String _getLibraryPath() {
  // Adjust paths based on your build setup
  // For development, you might point to the built library in target/
  if (Platform.isLinux) {
    return path.join(Directory.current.path, 'target', 'debug', 'libdart_todo.so');
  } else if (Platform.isMacOS) {
    return path.join(Directory.current.path, 'target', 'debug', 'libdart_todo.dylib');
  } else if (Platform.isWindows) {
    return path.join(Directory.current.path, 'target', 'debug', 'dart_todo.dll');
  } else {
    throw UnsupportedError('Platform not supported');
  }
}
```

Or use a simpler approach with dynamic loading (if library is in system path):

```dart
import 'dart:io';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'bindings/todo_bridge.dart';

/// Initialize the Rust library
Future<void> initRustLib() async {
  final libraryPath = _getLibraryPath();
  final externalLibrary = ExternalLibrary.open(libraryPath);
  await RustLib.init(externalLibrary: externalLibrary);
}

String _getLibraryPath() {
  if (Platform.isLinux) {
    return 'libdart_todo.so';
  } else if (Platform.isMacOS) {
    return 'libdart_todo.dylib';
  } else if (Platform.isWindows) {
    return 'dart_todo.dll';
  } else {
    throw UnsupportedError('Platform not supported');
  }
}
```

## Example: Using Todo in Dart

After the bridge is successfully set up, you can use Todo types and functions directly in Dart:

### Example 1: Creating a Todo

```dart
import 'package:todo_dart/bindings/todo_bridge.dart';

void createTodoExample() async {
  try {
    // Create a new todo
    final todo = await createTodo(description: 'Buy groceries');
    
    print('Created todo: ${todo.id}');
    print('Description: ${todo.description}');
    print('State: ${todo.state}'); // TodoState.todo
    print('Created at: ${todo.createdAt}');
  } on TodoError catch (e) {
    if (e == TodoError.emptyDescription) {
      print('Error: Description cannot be empty');
    }
  }
}
```

### Example 2: Creating a Todo with Events

```dart
void createTodoWithEventsExample() async {
  try {
    final result = await createTodoWithEvents(description: 'Complete project');
    final todo = result.$1;
    final events = result.$2;
    
    print('Todo created: ${todo.id}');
    print('Events generated: ${events.length}');
    
    for (final event in events) {
      if (event is TodoEventTodoCreated) {
        print('Event: TodoCreated - ${event.id}');
      }
    }
  } on TodoError catch (e) {
    print('Error: $e');
  }
}
```

### Example 3: Changing Todo State

```dart
void changeTodoStateExample() async {
  try {
    // Create a todo
    var todo = await createTodo(description: 'Write documentation');
    
    // Change to next state (Todo -> InProgress)
    final events1 = await todoChangeToNextState(todo: todo);
    print('State changed to: ${todo.state}'); // TodoState.inProgress
    
    // Change to next state again (InProgress -> Done)
    final events2 = await todoChangeToNextState(todo: todo);
    print('State changed to: ${todo.state}'); // TodoState.done
    
    // Try to change to next state (should fail - already Done)
    try {
      await todoChangeToNextState(todo: todo);
    } on TodoError catch (e) {
      if (e == TodoError.invalidStateTransition) {
        print('Cannot advance from Done state');
      }
    }
  } on TodoError catch (e) {
    print('Error: $e');
  }
}
```

### Example 4: Using TodoState Enum

```dart
void todoStateExample() async {
  var todo = await createTodo(description: 'Learn Flutter Rust Bridge');
  
  // Check current state
  switch (todo.state) {
    case TodoState.todo:
      print('Todo is in initial state');
      break;
    case TodoState.inProgress:
      print('Todo is in progress');
      break;
    case TodoState.done:
      print('Todo is completed');
      break;
  }
  
  // Update to specific state
  final events = await todoUpdateState(
    todo: todo,
    newState: TodoState.inProgress,
  );
  
  print('Updated to: ${todo.state}');
}
```

### Example 5: Handling TodoError

```dart
void errorHandlingExample() async {
  // Try to create todo with empty description
  try {
    await createTodo(description: '');
  } on TodoError catch (e) {
    switch (e) {
      case TodoError.emptyDescription:
        print('Description cannot be empty');
        break;
      case TodoError.invalidStateTransition:
        print('Invalid state transition');
        break;
      case TodoError.todoNotFound:
        print('Todo not found');
        break;
    }
  }
  
  // Try invalid state transition
  var todo = await createTodo(description: 'Test');
  await todoChangeToNextState(todo: todo); // Todo -> InProgress
  await todoChangeToNextState(todo: todo); // InProgress -> Done
  
  try {
    await todoChangeToNextState(todo: todo); // Should fail
  } on TodoError catch (e) {
    if (e == TodoError.invalidStateTransition) {
      print('Cannot advance from Done state');
    }
  }
}
```

### Example 6: Working with TodoEvent

```dart
void todoEventExample() async {
  // Create todo and get events
  final result = await createTodoWithEvents(description: 'Build app');
  final todo = result.$1;
  final events = result.$2;
  
  // Process events
  for (final event in events) {
    if (event is TodoEventTodoCreated) {
      print('TodoCreated event:');
      print('  ID: ${event.id}');
      print('  Description: ${event.description}');
      print('  Created at: ${event.createdAt}');
    }
  }
  
  // Change state and get events
  final stateChangeEvents = await todoChangeToNextState(todo: todo);
  
  for (final event in stateChangeEvents) {
    if (event is TodoEventTodoStateChanged) {
      print('TodoStateChanged event:');
      print('  ID: ${event.id}');
      print('  From: ${event.fromState}');
      print('  To: ${event.toState}');
      print('  Changed at: ${event.changedAt}');
    }
  }
}
```

### Example 7: Complete Dart CLI Example

```dart
import 'dart:io';
import 'package:todo_dart/bindings/todo_bridge.dart';
import 'package:todo_dart/todo.dart'; // Contains initRustLib()

/// Todo Manager - A simple CLI application for managing todos
class TodoManager {
  List<Todo> todos = [];
  
  Future<void> initialize() async {
    await initRustLib();
    await loadTodos();
  }
  
  Future<void> loadTodos() async {
    // In a real app, you would use a repository/handler
    // For now, we'll create a few example todos
    final todo1 = await createTodo(description: 'Learn Dart Rust Bridge');
    final todo2 = await createTodo(description: 'Build Todo app');
    
    todos = [todo1, todo2];
    print('Loaded ${todos.length} todos');
  }
  
  Future<void> addTodo(String description) async {
    try {
      final todo = await createTodo(description: description);
      todos.add(todo);
      print('✓ Added todo: ${todo.description}');
    } on TodoError catch (e) {
      if (e == TodoError.emptyDescription) {
        print('✗ Error: Description cannot be empty');
      } else {
        print('✗ Error: $e');
      }
    }
  }
  
  Future<void> advanceTodoState(int index) async {
    if (index < 0 || index >= todos.length) {
      print('✗ Invalid todo index');
      return;
    }
    
    final todo = todos[index];
    try {
      await todoChangeToNextState(todo: todo);
      print('✓ Advanced todo "${todo.description}" to ${getStateText(todo.state)}');
    } on TodoError catch (e) {
      if (e == TodoError.invalidStateTransition) {
        print('✗ Cannot advance from current state');
      } else {
        print('✗ Error: $e');
      }
    }
  }
  
  Future<void> updateTodoState(int index, TodoState newState) async {
    if (index < 0 || index >= todos.length) {
      print('✗ Invalid todo index');
      return;
    }
    
    final todo = todos[index];
    try {
      await todoUpdateState(todo: todo, newState: newState);
      print('✓ Updated todo "${todo.description}" to ${getStateText(todo.state)}');
    } on TodoError catch (e) {
      print('✗ Error: $e');
    }
  }
  
  void listTodos() {
    if (todos.isEmpty) {
      print('No todos found');
      return;
    }
    
    print('\n=== Todo List ===');
    for (int i = 0; i < todos.length; i++) {
      final todo = todos[i];
      final stateIcon = _getStateIcon(todo.state);
      print('$i. $stateIcon ${todo.description} [${getStateText(todo.state)}]');
      print('   ID: ${todo.id}');
      print('   Created: ${todo.createdAt}');
    }
    print('');
  }
  
  String getStateText(TodoState state) {
    switch (state) {
      case TodoState.todo:
        return 'Todo';
      case TodoState.inProgress:
        return 'In Progress';
      case TodoState.done:
        return 'Done';
    }
  }
  
  String _getStateIcon(TodoState state) {
    switch (state) {
      case TodoState.todo:
        return '○';
      case TodoState.inProgress:
        return '◐';
      case TodoState.done:
        return '✓';
    }
  }
}

/// Main entry point for CLI application
Future<void> main() async {
  final manager = TodoManager();
  
  try {
    await manager.initialize();
    
    // Interactive CLI loop
    print('=== Todo Manager ===');
    print('Commands:');
    print('  add <description>  - Add a new todo');
    print('  list               - List all todos');
    print('  advance <index>    - Advance todo state');
    print('  quit               - Exit');
    print('');
    
    while (true) {
      stdout.write('> ');
      final input = stdin.readLineSync();
      if (input == null || input.trim().isEmpty) continue;
      
      final parts = input.trim().split(' ');
      final command = parts[0].toLowerCase();
      
      switch (command) {
        case 'add':
          if (parts.length < 2) {
            print('Usage: add <description>');
            break;
          }
          final description = parts.sublist(1).join(' ');
          await manager.addTodo(description);
          break;
          
        case 'list':
          manager.listTodos();
          break;
          
        case 'advance':
          if (parts.length < 2) {
            print('Usage: advance <index>');
            break;
          }
          final index = int.tryParse(parts[1]);
          if (index == null) {
            print('Invalid index');
            break;
          }
          await manager.advanceTodoState(index);
          break;
          
        case 'quit':
        case 'exit':
          print('Goodbye!');
          exit(0);
          
        default:
          print('Unknown command: $command');
      }
    }
  } catch (e, stackTrace) {
    print('Error: $e');
    print('Stack trace: $stackTrace');
    exit(1);
  }
}
```

## Type Mappings

flutter_rust_bridge automatically maps Rust types to Dart types:

| Rust Type | Dart Type |
|-----------|-----------|
| `String` | `String` |
| `Vec<T>` | `List<T>` |
| `Option<T>` | `T?` (nullable) |
| `Result<T, E>` | `T` (throws `E` as exception) |
| `enum` | `enum class` |
| `struct` | `class` |
| `DateTime<Utc>` | `DateTime` |
| `&mut T` | `T` (mutable reference handled automatically) |

## Building and Running

### Build Rust Library

```bash
cd crates/dart-todo
cargo build --release
```

### Generate Dart Bindings

```bash
flutter_rust_bridge_codegen generate
```

### Run Dart Application

```bash
cd dart
dart run lib/todo.dart
```

Or run tests:

```bash
dart test
```

## Platform-Specific Notes

### Desktop (Linux/macOS/Windows)

- Place the compiled library (`.so`, `.dylib`, or `.dll`) in the appropriate location
- Update library loading path in Dart code
- For CLI applications, ensure the library is in the same directory or in the system library path

## Troubleshooting

### Common Issues

1. **Library not found**: Ensure the compiled Rust library is in the correct location for your platform
2. **Type mismatches**: Check that your Rust types are properly marked with `#[frb]` attributes
3. **Async issues**: Ensure async functions are properly marked with `#[frb(async)]`
4. **Build errors**: Make sure `crate-type` includes `"cdylib"` in `Cargo.toml`

## References

- [flutter_rust_bridge Documentation](https://cjycode.com/flutter_rust_bridge/)
- [Flutter Rust Bridge GitHub](https://github.com/fzyzcjy/flutter_rust_bridge)
- [Quickstart Guide](https://cjycode.com/flutter_rust_bridge/quickstart.html)

