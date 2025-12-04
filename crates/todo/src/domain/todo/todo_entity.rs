use chrono::{DateTime, Utc};
use crate::domain::todo::{TodoError, TodoEvent, TodoState};

#[cfg(feature = "dart")]
use flutter_rust_bridge::frb;

/// Aggregate root representing a Todo task
#[cfg_attr(feature = "dart", frb(non_opaque))]
pub struct Todo {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub description: String,
    pub state: TodoState,
}

#[cfg_attr(feature = "dart", frb(ignore))]
impl Todo {
    /// Creates a new Todo instance
    /// 
    /// # Parameters
    /// - `description`: Task description (must be non-empty)
    /// 
    /// # Returns
    /// - `Ok((Todo, Vec<TodoEvent>))`: Returns new Todo and `[TodoEvent::TodoCreated]`
    /// - `Err(TodoError::EmptyDescription)`: If description is empty
    /// 
    /// # Special Requirements
    /// - Validates non-empty description
    /// - Generates unique ID
    /// - Sets `state = TodoState::Todo`
    /// - Sets `created_at` to current timestamp
    pub fn new(description: String) -> Result<(Self, Vec<TodoEvent>), TodoError> {
        if description.trim().is_empty() {
            return Err(TodoError::EmptyDescription);
        }

        let id = uuid::Uuid::new_v4().to_string();
        let created_at = Utc::now();

        let todo = Todo {
            id: id.clone(),
            created_at,
            description: description.clone(),
            state: TodoState::Todo,
        };

        let event = TodoEvent::TodoCreated {
            id,
            description,
            created_at,
        };

        Ok((todo, vec![event]))
    }

    /// Updates the Todo state with validation
    /// 
    /// # Parameters
    /// - `&mut self`: Mutable reference to Todo (mutable pattern)
    /// - `new_state`: Target state to transition to
    /// 
    /// # Returns
    /// - `Ok(Vec<TodoEvent>)`: Returns `[TodoEvent::TodoStateChanged]`
    /// - `Err(TodoError::InvalidStateTransition)`: If transition not allowed or same state
    /// 
    /// # Special Requirements
    /// - Validates new state differs from current using TodoState::can_transition_to()
    /// - Mutates internal state directly
    /// - Marks as `dirty`
    pub fn update_state(&mut self, new_state: TodoState) -> Result<Vec<TodoEvent>, TodoError> {
        if !self.state.can_transition_to(new_state) {
            return Err(TodoError::InvalidStateTransition);
        }

        let from_state = self.state;
        let changed_at = Utc::now();

        self.state = new_state;

        let event = TodoEvent::TodoStateChanged {
            id: self.id.clone(),
            from_state,
            to_state: new_state,
            changed_at,
        };

        Ok(vec![event])
    }

    /// Transitions to the next state in the workflow
    /// 
    /// # Parameters
    /// - `&mut self`: Mutable reference to Todo (mutable pattern)
    /// 
    /// # Returns
    /// - `Ok(Vec<TodoEvent>)`: Returns `[TodoEvent::TodoStateChanged]`
    /// - `Err(TodoError::InvalidStateTransition)`: If already `Done` (cannot advance further)
    /// 
    /// # Special Requirements
    /// - Transitions: `Todo` → `InProgress` → `Done`
    /// - Mutates internal state directly
    /// - Marks as `dirty`
    pub fn change_to_next_state(&mut self) -> Result<Vec<TodoEvent>, TodoError> {
        let next_state = match self.state {
            TodoState::Todo => TodoState::InProgress,
            TodoState::InProgress => TodoState::Done,
            TodoState::Done => return Err(TodoError::InvalidStateTransition),
        };

        self.update_state(next_state)
    }

    /// Transitions to the previous state in the workflow
    /// 
    /// # Parameters
    /// - `&mut self`: Mutable reference to Todo (mutable pattern)
    /// 
    /// # Returns
    /// - `Ok(Vec<TodoEvent>)`: Returns `[TodoEvent::TodoStateChanged]`
    /// - `Err(TodoError::InvalidStateTransition)`: If already `Todo` (cannot retreat further)
    /// 
    /// # Special Requirements
    /// - Transitions: `Done` → `InProgress` → `Todo`
    /// - Mutates internal state directly
    /// - Marks as `dirty`
    pub fn change_to_previous_state(&mut self) -> Result<Vec<TodoEvent>, TodoError> {
        let previous_state = match self.state {
            TodoState::Done => TodoState::InProgress,
            TodoState::InProgress => TodoState::Todo,
            TodoState::Todo => return Err(TodoError::InvalidStateTransition),
        };

        self.update_state(previous_state)
    }
}

