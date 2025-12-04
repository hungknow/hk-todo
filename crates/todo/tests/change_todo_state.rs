mod common;

use todo::application::change_todo_state_handler::ChangeTodoStateHandler;
use todo::infrastructure::repositories::todo::InMemoryTodoRepository;
use todo::{Todo, TodoError, TodoEvent, TodoState};
use common::MockTodoRepository;

/// Test case structure for state transition tests
struct StateTransitionTestCase {
    name: &'static str,
    initial_state: TodoState,
    target_state: TodoState,
    expected_result: TestResult,
}

/// Expected result of a state transition test
enum TestResult {
    Success {
        expected_from_state: TodoState,
        expected_to_state: TodoState,
    },
    Error(TodoError),
}

/// Helper function to create a todo in a specific initial state
fn create_todo_in_state(initial_state: TodoState) -> (Todo, String) {
    let (mut todo, _) = Todo::new("Test todo".to_string()).unwrap();
    let todo_id = todo.id.clone();
    
    // Transition to initial state if not already Todo
    match initial_state {
        TodoState::Todo => {
            // Already in Todo state, no transition needed
        }
        TodoState::InProgress => {
            todo.update_state(TodoState::InProgress).unwrap();
        }
        TodoState::Done => {
            todo.update_state(TodoState::InProgress).unwrap();
            todo.update_state(TodoState::Done).unwrap();
        }
    }
    
    (todo, todo_id)
}

/// Helper function to run a state transition test
async fn run_state_transition_test(test_case: StateTransitionTestCase) {
    // Arrange
    let repository = Box::new(InMemoryTodoRepository::new()) as Box<dyn todo::TodoRepository>;
    let (todo, todo_id) = create_todo_in_state(test_case.initial_state);
    repository.save(&todo).unwrap();
    let handler = ChangeTodoStateHandler::new(repository);

    // Act
    let result = handler.change_state(todo_id.clone(), test_case.target_state).await;

    // Assert
    match test_case.expected_result {
        TestResult::Success {
            expected_from_state,
            expected_to_state,
        } => {
            assert!(result.is_ok(), "Test '{}' should succeed", test_case.name);
            let events = result.unwrap();
            assert_eq!(events.len(), 1, "Test '{}' should produce one event", test_case.name);
            
            match &events[0] {
                TodoEvent::TodoStateChanged {
                    id,
                    from_state,
                    to_state,
                    changed_at: _,
                } => {
                    assert_eq!(id, &todo_id, "Test '{}' event should have correct id", test_case.name);
                    assert_eq!(
                        *from_state, expected_from_state,
                        "Test '{}' should transition from {:?}",
                        test_case.name, expected_from_state
                    );
                    assert_eq!(
                        *to_state, expected_to_state,
                        "Test '{}' should transition to {:?}",
                        test_case.name, expected_to_state
                    );
                }
                _ => panic!("Test '{}' expected TodoStateChanged event", test_case.name),
            }
        }
        TestResult::Error(expected_error) => {
            assert!(result.is_err(), "Test '{}' should fail", test_case.name);
            let actual_error = result.unwrap_err();
            assert_eq!(
                actual_error, expected_error,
                "Test '{}' should return {:?}",
                test_case.name, expected_error
            );
        }
    }
}

#[tokio::test]
async fn test_change_state_todo_to_in_progress_success() {
    run_state_transition_test(StateTransitionTestCase {
        name: "Todo to InProgress",
        initial_state: TodoState::Todo,
        target_state: TodoState::InProgress,
        expected_result: TestResult::Success {
            expected_from_state: TodoState::Todo,
            expected_to_state: TodoState::InProgress,
        },
    })
    .await;
}

#[tokio::test]
async fn test_change_state_in_progress_to_done_success() {
    run_state_transition_test(StateTransitionTestCase {
        name: "InProgress to Done",
        initial_state: TodoState::InProgress,
        target_state: TodoState::Done,
        expected_result: TestResult::Success {
            expected_from_state: TodoState::InProgress,
            expected_to_state: TodoState::Done,
        },
    })
    .await;
}

#[tokio::test]
async fn test_change_state_done_to_in_progress_success() {
    run_state_transition_test(StateTransitionTestCase {
        name: "Done to InProgress",
        initial_state: TodoState::Done,
        target_state: TodoState::InProgress,
        expected_result: TestResult::Success {
            expected_from_state: TodoState::Done,
            expected_to_state: TodoState::InProgress,
        },
    })
    .await;
}

#[tokio::test]
async fn test_change_state_invalid_transition_error() {
    run_state_transition_test(StateTransitionTestCase {
        name: "Invalid transition (Todo to Done)",
        initial_state: TodoState::Todo,
        target_state: TodoState::Done,
        expected_result: TestResult::Error(TodoError::InvalidStateTransition),
    })
    .await;
}

#[tokio::test]
async fn test_change_state_same_state_error() {
    run_state_transition_test(StateTransitionTestCase {
        name: "Same state transition",
        initial_state: TodoState::Todo,
        target_state: TodoState::Todo,
        expected_result: TestResult::Error(TodoError::InvalidStateTransition),
    })
    .await;
}

#[tokio::test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
async fn test_change_state_todo_not_found_panics() {
    // Arrange - Mock repository that returns None
    let repository = Box::new(MockTodoRepository::new(true)) as Box<dyn todo::TodoRepository>;
    let handler = ChangeTodoStateHandler::new(repository);

    // Act - This will panic because find_by_id returns None and handler uses .unwrap()
    let _result = handler.change_state("non-existent-id".to_string(), TodoState::InProgress).await;
}
