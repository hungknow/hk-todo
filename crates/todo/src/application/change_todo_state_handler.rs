use crate::{TodoError, TodoEvent, TodoRepository, TodoState};

pub struct ChangeTodoStateHandler {
    todo_repository: Box<dyn TodoRepository>,
}

impl ChangeTodoStateHandler {
    pub fn new(todo_repository: Box<dyn TodoRepository>) -> Self {
        Self { todo_repository }
    }

    pub async fn change_state(&self, id: String, new_state: TodoState) -> Result<Vec<TodoEvent>, TodoError> {
        let mut todo = self.todo_repository.find_by_id(&id).await?.unwrap();
        let events = todo.update_state(new_state)?;
        self.todo_repository.save(&todo).await?;
        Ok(events)
    }
}
