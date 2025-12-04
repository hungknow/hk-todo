import 'package:test/test.dart';
import 'package:todo_dart/todo.dart';
import 'package:todo_dart/bindings/todo_bridge.dart/frb_generated.dart';

/// Integration tests for AddTodoHandler functionality
/// Tests the domain functions that correspond to AddTodoHandler: create_todo and create_todo_with_events
void main() {
  setUpAll(() async {
    // Initialize the Rust library before running tests
    await initRustLib();
  });

  group('AddTodoHandler Integration Tests', () {
    test('Todo constructor should successfully create a todo with valid description', () {
      // Arrange
      const description = 'Test todo description';

      // Act
      final todo = Todo(description: description);

      // Assert
      expect(todo, isNotNull);
      expect(todo, isA<Todo>());
    });

    test('Todo constructor should throw error for empty description', () {
      // Arrange
      const emptyDescription = '';

      // Act & Assert
      expect(
        () => Todo(description: emptyDescription),
        throwsA(isA<TodoError>()),
      );
    });

    test('Todo constructor should throw error for whitespace-only description', () {
      // Arrange
      const whitespaceDescription = '   ';

      // Act & Assert
      expect(
        () => Todo(description: whitespaceDescription),
        throwsA(isA<TodoError>()),
      );
    });

    test('createTodoWithEvents should return todo and events', () {
      // Arrange
      const description = 'Test todo with events';

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

    test('createTodoWithEvents should throw error for empty description', () {
      // Arrange
      const emptyDescription = '';

      // Act & Assert
      expect(
        () => RustLib.instance.api.crateCreateTodoWithEvents(description: emptyDescription),
        throwsA(isA<TodoError>()),
      );
    });

    test('Todo constructor should generate unique todos', () {
      // Arrange
      const description1 = 'First todo';
      const description2 = 'Second todo';

      // Act
      final todo1 = Todo(description: description1);
      final todo2 = Todo(description: description2);

      // Assert
      expect(todo1, isNotNull);
      expect(todo2, isNotNull);
      expect(todo1, isNot(same(todo2)));
    });

    test('Todo constructor should create valid todo instances', () {
      // Arrange
      const description = 'New todo item';

      // Act
      final todo = Todo(description: description);

      // Assert
      expect(todo, isNotNull);
      expect(todo, isA<Todo>());
    });
  });
}
