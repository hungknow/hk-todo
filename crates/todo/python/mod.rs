use pyo3::prelude::*;
use pyo3::types::PyModule;
use crate::{Todo, TodoState, TodoError, TodoEvent};
use chrono::{DateTime, Utc};

/// Python bindings for TodoState enum
#[pyclass]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PyTodoState {
    #[pyo3(name = "TODO")]
    Todo,
    #[pyo3(name = "IN_PROGRESS")]
    InProgress,
    #[pyo3(name = "DONE")]
    Done,
}

impl From<TodoState> for PyTodoState {
    fn from(state: TodoState) -> Self {
        match state {
            TodoState::Todo => PyTodoState::Todo,
            TodoState::InProgress => PyTodoState::InProgress,
            TodoState::Done => PyTodoState::Done,
        }
    }
}

impl From<PyTodoState> for TodoState {
    fn from(state: PyTodoState) -> Self {
        match state {
            PyTodoState::Todo => TodoState::Todo,
            PyTodoState::InProgress => TodoState::InProgress,
            PyTodoState::Done => TodoState::Done,
        }
    }
}

/// Python bindings for TodoError enum
#[pyclass]
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum PyTodoError {
    #[pyo3(name = "EMPTY_DESCRIPTION")]
    EmptyDescription,
    #[pyo3(name = "INVALID_STATE_TRANSITION")]
    InvalidStateTransition,
    #[pyo3(name = "TODO_NOT_FOUND")]
    TodoNotFound,
}

impl From<TodoError> for PyTodoError {
    fn from(err: TodoError) -> Self {
        match err {
            TodoError::EmptyDescription => PyTodoError::EmptyDescription,
            TodoError::InvalidStateTransition => PyTodoError::InvalidStateTransition,
            TodoError::TodoNotFound => PyTodoError::TodoNotFound,
        }
    }
}

impl From<PyTodoError> for TodoError {
    fn from(err: PyTodoError) -> Self {
        match err {
            PyTodoError::EmptyDescription => TodoError::EmptyDescription,
            PyTodoError::InvalidStateTransition => TodoError::InvalidStateTransition,
            PyTodoError::TodoNotFound => TodoError::TodoNotFound,
        }
    }
}

/// Python bindings for Todo struct
#[pyclass]
pub struct PyTodo {
    inner: Todo,
}

#[pymethods]
impl PyTodo {
    /// Creates a new Todo instance
    #[new]
    fn new(description: String) -> PyResult<(Self, Vec<PyTodoEvent>)> {
        let (todo, events) = Todo::new(description)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(
                format!("Failed to create todo: {:?}", e)
            ))?;
        
        let py_events: Vec<PyTodoEvent> = events.into_iter().map(|e| e.into()).collect();
        Ok((PyTodo { inner: todo }, py_events))
    }

    /// Get the todo ID
    #[getter]
    fn id(&self) -> String {
        self.inner.id.clone()
    }

    /// Get the todo description
    #[getter]
    fn description(&self) -> String {
        self.inner.description.clone()
    }

    /// Get the todo state
    #[getter]
    fn state(&self) -> PyTodoState {
        self.inner.state.into()
    }

    /// Get the creation timestamp
    #[getter]
    fn created_at(&self) -> String {
        self.inner.created_at.to_rfc3339()
    }

    /// Updates the Todo state with validation
    fn update_state(&mut self, new_state: PyTodoState) -> PyResult<Vec<PyTodoEvent>> {
        let state: TodoState = new_state.into();
        let events = self.inner.update_state(state)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(
                format!("Failed to update state: {:?}", e)
            ))?;
        
        Ok(events.into_iter().map(|e| e.into()).collect())
    }

    /// Transitions to the next state in the workflow
    fn change_to_next_state(&mut self) -> PyResult<Vec<PyTodoEvent>> {
        let events = self.inner.change_to_next_state()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(
                format!("Failed to change to next state: {:?}", e)
            ))?;
        
        Ok(events.into_iter().map(|e| e.into()).collect())
    }

    /// Transitions to the previous state in the workflow
    fn change_to_previous_state(&mut self) -> PyResult<Vec<PyTodoEvent>> {
        let events = self.inner.change_to_previous_state()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(
                format!("Failed to change to previous state: {:?}", e)
            ))?;
        
        Ok(events.into_iter().map(|e| e.into()).collect())
    }
}

/// Python bindings for TodoEvent enum
#[pyclass]
#[derive(Clone, Debug)]
pub enum PyTodoEvent {
    #[pyo3(name = "TODO_CREATED")]
    TodoCreated {
        id: String,
        description: String,
        created_at: String,
    },
    #[pyo3(name = "TODO_STATE_CHANGED")]
    TodoStateChanged {
        id: String,
        from_state: PyTodoState,
        to_state: PyTodoState,
        changed_at: String,
    },
}

impl From<TodoEvent> for PyTodoEvent {
    fn from(event: TodoEvent) -> Self {
        match event {
            TodoEvent::TodoCreated { id, description, created_at } => {
                PyTodoEvent::TodoCreated {
                    id,
                    description,
                    created_at: created_at.to_rfc3339(),
                }
            }
            TodoEvent::TodoStateChanged { id, from_state, to_state, changed_at } => {
                PyTodoEvent::TodoStateChanged {
                    id,
                    from_state: from_state.into(),
                    to_state: to_state.into(),
                    changed_at: changed_at.to_rfc3339(),
                }
            }
        }
    }
}

/// Python module initialization
#[pymodule]
fn todo(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyTodo>()?;
    m.add_class::<PyTodoState>()?;
    m.add_class::<PyTodoError>()?;
    m.add_class::<PyTodoEvent>()?;
    Ok(())
}

