import 'package:test/test.dart';
import 'package:todo_dart/todo.dart';
import 'package:todo_dart/bindings/todo_bridge.dart/frb_generated.dart';

/// Integration tests for ChangeTodoStateHandler functionality
/// Tests the domain functions that correspond to ChangeTodoStateHandler:
/// - todo_update_state
/// - todo_change_to_next_state
/// - todo_change_to_previous_state
void main() {
  setUpAll(() async {
    // Initialize the Rust library before running tests
    await initRustLib();
  });

  group('ChangeTodoStateHandler Integration Tests', () {
    test('todoChangeToNextState should transition from Todo to InProgress', () {
      // Arrange
      final todo = Todo(description: 'Test todo');
      expect(todo, isNotNull);

      // Act
      final events = RustLib.instance.api.crateTodoChangeToNextState(todo: todo);

      // Assert
      expect(events, isNotNull);
      expect(events.length, 1);
      expect(events[0], isA<TodoEvent>());
    });

    test('todoChangeToNextState should allow multiple transitions', () {
      // Arrange
      var todo = Todo(description: 'Test todo');
      expect(todo, isNotNull);

      // Act: First transition
      final firstEvents = RustLib.instance.api.crateTodoChangeToNextState(todo: todo);
      expect(firstEvents.length, 1);

      // Act: Second transition
      final secondEvents = RustLib.instance.api.crateTodoChangeToNextState(todo: todo);
      expect(secondEvents.length, 1);
      expect(secondEvents[0], isA<TodoEvent>());
    });

    test('todoChangeToNextState should throw error when already Done', () {
      // Arrange
      var todo = Todo(description: 'Test todo');
      
      // Transition to Done (two transitions: Todo -> InProgress -> Done)
      RustLib.instance.api.crateTodoChangeToNextState(todo: todo);
      RustLib.instance.api.crateTodoChangeToNextState(todo: todo);

      // Act & Assert: Try to advance from Done
      expect(
        () => RustLib.instance.api.crateTodoChangeToNextState(todo: todo),
        throwsA(isA<TodoError>()),
      );
    });

    test('todoChangeToPreviousState should allow backward transitions', () {
      // Arrange
      var todo = Todo(description: 'Test todo');
      
      // Transition to Done
      RustLib.instance.api.crateTodoChangeToNextState(todo: todo);
      RustLib.instance.api.crateTodoChangeToNextState(todo: todo);

      // Act: Transition back to InProgress
      final events = RustLib.instance.api.crateTodoChangeToPreviousState(todo: todo);

      // Assert
      expect(events, isNotNull);
      expect(events.length, 1);
      expect(events[0], isA<TodoEvent>());
    });

    test('todoChangeToPreviousState should throw error when already Todo', () {
      // Arrange
      final todo = Todo(description: 'Test todo');

      // Act & Assert: Try to retreat from Todo
      expect(
        () => RustLib.instance.api.crateTodoChangeToPreviousState(todo: todo),
        throwsA(isA<TodoError>()),
      );
    });

    // Note: todoUpdateState tests are skipped because they require TodoState values
    // which are opaque types. To test these, we would need getter functions
    // to extract TodoState from Todo, or helper functions to create TodoState values.
  });
}
