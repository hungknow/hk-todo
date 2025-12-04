use std::sync::{Arc, Mutex};
use todo::{Todo, TodoError, TodoRepository};

/// Mock repository for testing scenarios where we need to control repository behavior
pub struct MockTodoRepository {
    should_return_none: Arc<Mutex<bool>>,
}

impl MockTodoRepository {
    /// Creates a new MockTodoRepository
    /// 
    /// # Parameters
    /// - `should_return_none`: If `true`, `find_by_id` will return `None`. If `false`, it will return a test todo.
    pub fn new(should_return_none: bool) -> Self {
        Self {
            should_return_none: Arc::new(Mutex::new(should_return_none)),
        }
    }
}

impl TodoRepository for MockTodoRepository {
    fn save(&self, _todo: &Todo) -> Result<(), TodoError> {
        Ok(())
    }

    fn find_by_id(&self, _id: &str) -> Result<Option<Todo>, TodoError> {
        let should_return_none = *self.should_return_none.lock().unwrap();
        if should_return_none {
            Ok(None)
        } else {
            let (todo, _) = Todo::new("Test todo".to_string()).unwrap();
            Ok(Some(todo))
        }
    }

    fn find_all(&self) -> Result<Vec<Todo>, TodoError> {
        Ok(vec![])
    }

    fn delete(&self, _id: &str) -> Result<(), TodoError> {
        Ok(())
    }
}

