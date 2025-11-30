use todo::application::add_todo_handler::AddTodoHandler;
use todo::infrastructure::repositories::todo::InMemoryTodoRepository;
use todo::{TodoError, TodoEvent};

#[tokio::test]
async fn test_add_todo_success() {
    // Arrange
    let repository = Box::new(InMemoryTodoRepository::new()) as Box<dyn todo::TodoRepository>;
    let handler = AddTodoHandler::new(repository);
    let description = "Test todo description".to_string();

    // Act
    let events = handler.new_todo(description.clone()).await;

    // Assert
    assert!(events.is_ok());
    let events = events.unwrap();
    assert_eq!(events.len(), 1);
    
    match &events[0] {
        TodoEvent::TodoCreated {
            id,
            description: event_description,
            created_at,
        } => {
            assert!(!id.is_empty());
            assert_eq!(event_description, &description);
            assert!(!created_at.to_string().is_empty());
        }
        _ => panic!("Expected TodoCreated event"),
    }
}

#[tokio::test]
async fn test_add_todo_empty_description_error() {
    // Arrange
    let repository = Box::new(InMemoryTodoRepository::new()) as Box<dyn todo::TodoRepository>;
    let handler = AddTodoHandler::new(repository);
    let empty_description = "".to_string();

    // Act
    let result = handler.new_todo(empty_description).await;

    // Assert
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), TodoError::EmptyDescription));
}

#[tokio::test]
async fn test_add_todo_whitespace_only_description_error() {
    // Arrange
    let repository = Box::new(InMemoryTodoRepository::new()) as Box<dyn todo::TodoRepository>;
    let handler = AddTodoHandler::new(repository);
    let whitespace_description = "   ".to_string();

    // Act
    let result = handler.new_todo(whitespace_description).await;

    // Assert
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), TodoError::EmptyDescription));
}

