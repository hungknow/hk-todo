use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::domain::todo::{Todo, TodoError, TodoRepository};

#[cfg(feature = "dart")]
use flutter_rust_bridge::frb;

/// In-memory implementation of TodoRepository
/// 
/// This implementation stores todos in a HashMap wrapped in Arc<RwLock> for thread-safe access.
/// Todos are stored by their ID and can be retrieved, updated, or deleted.
#[cfg_attr(feature = "dart", frb(opaque))]
pub struct InMemoryTodoRepository {
    todos: Arc<RwLock<HashMap<String, Todo>>>,
}

impl InMemoryTodoRepository {
    /// Creates a new InMemoryTodoRepository instance
    pub fn new() -> Self {
        InMemoryTodoRepository {
            todos: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryTodoRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg_attr(feature = "dart", frb(ignore))]
impl TodoRepository for InMemoryTodoRepository {
    fn save(&self, todo: &Todo) -> Result<(), TodoError> {
        let mut todos = self.todos.write().map_err(|_| {
            // In practice, this shouldn't happen, but we need to handle the error
            // For simplicity, we'll use a generic error. In a real implementation,
            // you might want a specific error type for lock poisoning.
            TodoError::TodoNotFound
        })?;
        
        // Clone the todo to store it (insert or update)
        // Since Todo doesn't implement Clone, we'll reconstruct it from its fields
        let todo_to_store = Todo {
            id: todo.id.clone(),
            created_at: todo.created_at,
            description: todo.description.clone(),
            state: todo.state,
        };
        
        todos.insert(todo.id.clone(), todo_to_store);
        Ok(())
    }

    fn find_by_id(&self, id: &str) -> Result<Option<Todo>, TodoError> {
        let todos = self.todos.read().map_err(|_| {
            TodoError::TodoNotFound
        })?;
        
        match todos.get(id) {
            Some(todo) => {
                // Clone the todo to return it
                Ok(Some(Todo {
                    id: todo.id.clone(),
                    created_at: todo.created_at,
                    description: todo.description.clone(),
                    state: todo.state,
                }))
            }
            None => Ok(None),
        }
    }

    fn find_all(&self) -> Result<Vec<Todo>, TodoError> {
        let todos = self.todos.read().map_err(|_| {
            TodoError::TodoNotFound
        })?;
        
        let result: Vec<Todo> = todos
            .values()
            .map(|todo| Todo {
                id: todo.id.clone(),
                created_at: todo.created_at,
                description: todo.description.clone(),
                state: todo.state,
            })
            .collect();
        
        Ok(result)
    }

    fn delete(&self, id: &str) -> Result<(), TodoError> {
        let mut todos = self.todos.write().map_err(|_| {
            TodoError::TodoNotFound
        })?;
        
        todos.remove(id);
        Ok(())
    }
}

