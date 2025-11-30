use crate::{Todo, TodoError, TodoRepository};

pub struct GetTodosHandler {
    todo_repository: Box<dyn TodoRepository>,
}

impl GetTodosHandler {
    pub fn new(todo_repository: Box<dyn TodoRepository>) -> Self {
        Self { todo_repository }
    }

    pub async fn get_todos(&self) -> Result<Vec<Todo>, TodoError> {
        let todos = self.todo_repository.find_all().await?;
        Ok(todos)
    }
}
