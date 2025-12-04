import 'package:test/test.dart';
import 'package:todo_dart/todo.dart';
import 'package:todo_dart/bindings/todo_bridge.dart/frb_generated.dart';

/// Integration tests for GetTodosHandler functionality
/// Since GetTodosHandler uses repository operations that aren't directly exposed
/// via Dart bindings, these tests verify the domain functions that would be used
/// by the handler to ensure todos can be created and their properties accessed correctly.
void main() {
  setUpAll(() async {
    // Initialize the Rust library before running tests
    await initRustLib();
  });

  group('GetTodosHandler Integration Tests', () {
    test('should be able to create multiple todos with different properties', () {
      // Arrange & Act
      final todo1 = Todo(description: 'First todo');
      final todo2 = Todo(description: 'Second todo');
      final todo3 = Todo(description: 'Third todo');

      // Assert
      expect(todo1, isNotNull);
      expect(todo2, isNotNull);
      expect(todo3, isNotNull);
      expect(todo1, isNot(same(todo2)));
      expect(todo1, isNot(same(todo3)));
      expect(todo2, isNot(same(todo3)));
    });

    test('should be able to create todos with events and access event properties', () {
      // Arrange
      const description = 'Todo with events';

      // Act
      final result = RustLib.instance.api.crateCreateTodoWithEvents(description: description);
      final todo = result.$1;
      final events = result.$2;

      // Assert
      expect(todo, isNotNull);
      expect(todo, isA<Todo>());
      expect(events, isNotNull);
      expect(events.length, 1);
      expect(events[0], isA<TodoEvent>());
    });

    test('should be able to track state changes through events', () {
      // Arrange
      var todo = Todo(description: 'State tracking test');
      expect(todo, isNotNull);

      // Act: Transition through states
      final toInProgressEvents = RustLib.instance.api.crateTodoChangeToNextState(todo: todo);
      expect(toInProgressEvents.length, 1);
      expect(toInProgressEvents[0], isA<TodoEvent>());

      final toDoneEvents = RustLib.instance.api.crateTodoChangeToNextState(todo: todo);
      expect(toDoneEvents.length, 1);
      expect(toDoneEvents[0], isA<TodoEvent>());
    });

    test('should maintain todo identity across state changes', () {
      // Arrange
      var todo = Todo(description: 'Identity test');

      // Act: Change state multiple times
      RustLib.instance.api.crateTodoChangeToNextState(todo: todo);
      RustLib.instance.api.crateTodoChangeToNextState(todo: todo);
      RustLib.instance.api.crateTodoChangeToPreviousState(todo: todo);

      // Assert: Todo object should still be valid
      expect(todo, isNotNull);
      expect(todo, isA<Todo>());
    });

    test('should handle multiple todos with different states', () {
      // Arrange & Act
      final todo1 = Todo(description: 'Todo 1');
      var todo2 = Todo(description: 'Todo 2');

      // Transition todo1 to InProgress
      RustLib.instance.api.crateTodoChangeToNextState(todo: todo1);

      // Transition todo2 to Done
      RustLib.instance.api.crateTodoChangeToNextState(todo: todo2);
      RustLib.instance.api.crateTodoChangeToNextState(todo: todo2);

      // Assert: Both todos should still be valid
      expect(todo1, isNotNull);
      expect(todo2, isNotNull);
      expect(todo1, isNot(same(todo2)));
    });
  });
}
