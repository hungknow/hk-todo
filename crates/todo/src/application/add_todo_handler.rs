use crate::{Todo, TodoError, TodoEvent, TodoRepository};

pub struct AddTodoHandler {
    todo_repository: Box<dyn TodoRepository>,
}

impl AddTodoHandler {
    pub fn new(todo_repository: Box<dyn TodoRepository>) -> Self {
        Self { todo_repository }
    }

    pub fn new_todo(&self, description: String) -> Result<Vec<TodoEvent>, TodoError> {
        let (todo, events) = Todo::new(description)?;
        self.todo_repository.save(&todo)?;
        Ok(events)
    }
}
