import 'package:todo_dart/todo.dart';
import 'package:todo_dart/bindings/todo_bridge.dart/frb_generated.dart';

/// Test helper utilities for Dart integration tests
class TestHelper {
  /// Initialize the Rust library for testing
  /// This should be called in setUpAll() of test groups
  static Future<void> setup() async {
    await RustLib.init();
  }

  /// Create a test todo with the given description
  /// Returns the created Todo or throws if creation fails
  static Todo createTestTodo(String description) {
    return Todo(description: description);
  }

  /// Create a test todo and transition it to InProgress
  static Todo createInProgressTodo(String description) {
    final todo = createTestTodo(description);
    RustLib.instance.api.crateTodoChangeToNextState(todo: todo);
    return todo;
  }

  /// Create a test todo and transition it to Done
  static Todo createDoneTodo(String description) {
    final todo = createInProgressTodo(description);
    RustLib.instance.api.crateTodoChangeToNextState(todo: todo);
    return todo;
  }
}
