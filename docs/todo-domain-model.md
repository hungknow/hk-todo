# Todo Domain Model Specification

## Overview

This document describes the Todo aggregate root following Domain-Driven Design (DDD) principles. The Todo aggregate is the core entity in the domain, representing a single task that can be tracked through different states in its lifecycle.

---

## File Structure

The following file tree describes the organization of domain-related files:

```
src/
├── domain/
│   └── todo/
│       ├── mod.rs                      # Public exports for Todo domain
│       ├── todo.entity.rs              # Entity, Enum (TodoState), and Todo methods
│       ├── todo.event.rs               # TodoEvent enum definition
│       ├── todos.collection.rs         # Todos collection struct and methods
│       └── todo.repository.rs          # Repository trait definition
│
├── infrastructure/
│   └── repositories/
│       └── todo/
│           ├── mod.rs                   # Public exports for Todo repository
│           └── sql_todo.repository.rs   # Repository implementation (TodoRepositoryPort)
```

---

## Structure

### Aggregate Root: Todo

The `Todo` is the aggregate root of the Todo bounded context. It encapsulates all business logic and invariants related to a single task.

#### Properties

```rust
pub struct Todo {
    pub id: String,                    // Unique identifier (immutable)
    pub created_at: DateTime<Utc>,     // Timestamp of creation (immutable)
    pub description: String,           // Task description (mutable)
    pub state: TodoState,              // Current state in workflow (mutable)
    pub(crate) dirty: Option<bool>,    // Flag indicating unsaved changes (internal)
}

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
        // ...
    }

    /// Updates the Todo state with validation
    /// 
    /// # Parameters
    /// - `self`: Takes ownership of Todo (immutable pattern)
    /// - `new_state`: Target state to transition to
    /// 
    /// # Returns
    /// - `Ok((Todo, Vec<TodoEvent>))`: Returns updated Todo and `[TodoEvent::TodoStateChanged]`
    /// - `Err(TodoError::InvalidStateTransition)`: If transition not allowed or same state
    /// 
    /// # Special Requirements
    /// - Validates new state differs from current
    /// - Marks as `dirty`
    /// - Returns new instance (immutable)
    pub fn update_state(self, new_state: TodoState) -> Result<(Self, Vec<TodoEvent>), TodoError> {
        // ...
    }

    /// Transitions to the next state in the workflow
    /// 
    /// # Parameters
    /// - `self`: Takes ownership of Todo (immutable pattern)
    /// 
    /// # Returns
    /// - `Ok((Todo, Vec<TodoEvent>))`: Returns updated Todo and `[TodoEvent::TodoStateChanged]`
    /// - `Err(TodoError::InvalidStateTransition)`: If already `Done` (cannot advance further)
    /// 
    /// # Special Requirements
    /// - Transitions: `Todo` → `InProgress` → `Done`
    /// - Marks as `dirty`
    pub fn change_to_next_state(self) -> Result<(Self, Vec<TodoEvent>), TodoError> {
        // ...
    }

    /// Transitions to the previous state in the workflow
    /// 
    /// # Parameters
    /// - `self`: Takes ownership of Todo (immutable pattern)
    /// 
    /// # Returns
    /// - `Ok((Todo, Vec<TodoEvent>))`: Returns updated Todo and `[TodoEvent::TodoStateChanged]`
    /// - `Err(TodoError::InvalidStateTransition)`: If already `Todo` (cannot retreat further)
    /// 
    /// # Special Requirements
    /// - Transitions: `Done` → `InProgress` → `Todo`
    /// - Marks as `dirty`
    pub fn change_to_previous_state(self) -> Result<(Self, Vec<TodoEvent>), TodoError> {
        // ...
    }

    /// Validates if a state transition is allowed
    /// 
    /// # Parameters
    /// - `self`: Reference to Todo
    /// - `new_state`: Target state to validate
    /// 
    /// # Returns
    /// - `bool`: `false` if same state, `true` if transition allowed
    /// 
    /// # Special Requirements
    /// - Extensible for workflow rules
    pub fn is_new_state_allowed(&self, new_state: TodoState) -> bool {
        // ...
    }
}
```

#### Value Objects

**TodoState** (Enum)
- `Todo`: Initial state when a todo is created
- `InProgress`: Intermediate state indicating work in progress
- `Done`: Final state indicating completion

The state follows a linear workflow: `Todo` → `InProgress` → `Done`

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TodoState {
    Todo,
    InProgress,
    Done,
}

impl TodoState {
    /// Checks if the state can advance to the next state
    /// 
    /// # Parameters
    /// - `self`: Reference to TodoState
    /// 
    /// # Returns
    /// - `bool`: `true` for `Todo`/`InProgress`, `false` for `Done`
    pub fn can_advance(&self) -> bool {
        matches!(self, TodoState::Todo | TodoState::InProgress)
    }

    /// Checks if the state can retreat to the previous state
    /// 
    /// # Parameters
    /// - `self`: Reference to TodoState
    /// 
    /// # Returns
    /// - `bool`: `false` for `Todo`, `true` for `InProgress`/`Done`
    pub fn can_retreat(&self) -> bool {
        matches!(self, TodoState::InProgress | TodoState::Done)
    }
}
```

#### Domain Events

**TodoEvent** (Enum)
Domain events that describe significant occurrences in the Todo lifecycle.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TodoEvent {
    TodoCreated {
        id: String,
        description: String,
        created_at: DateTime<Utc>,
    },
    TodoStateChanged {
        id: String,
        from_state: TodoState,
        to_state: TodoState,
        changed_at: DateTime<Utc>,
    },
}
```

Domain methods return domain events along with the modified Todo. Events are returned automatically when domain operations occur:
- `TodoCreated` - Returned when a new Todo is created via `Todo::new()`
- `TodoStateChanged` - Returned when the Todo state changes via `update_state()`, `change_to_next_state()`, or `change_to_previous_state()`

#### Invariants

1. **Description Invariant**: A Todo cannot have an empty or null description
2. **Identity Invariant**: The `id` and `created_at` are immutable once set
3. **State Consistency**: A Todo must always be in a valid state (one of the TodoState enum values)
4. **Event Consistency**: Events are returned in the order they occur and represent actual domain operations

---

### Todos Collection

The `Todos` struct is a collection aggregate that wraps multiple `Todo` instances and provides collection-level domain operations.

#### Properties

```rust
pub struct Todos {
    items: HashMap<String, Todo>,
}

impl Todos {
    /// Creates a new empty Todos collection
    /// 
    /// # Returns
    /// - `Todos`: Empty collection with initialized HashMap
    pub fn new() -> Self {
        // ...
    }

    /// Creates and adds a new Todo to the collection
    /// 
    /// # Parameters
    /// - `self`: Takes ownership of Todos (immutable pattern)
    /// - `description`: Task description for the new Todo
    /// 
    /// # Returns
    /// - `Ok((Todos, Vec<TodoEvent>))`: Returns updated Todos and `[TodoEvent::TodoCreated]`
    /// - `Err(TodoError::EmptyDescription)`: If description is empty
    /// 
    /// # Special Requirements
    /// - Creates Todo via `Todo::new()`
    /// - Uses generated Todo ID as HashMap key
    /// - Preserves existing Todos (immutable)
    /// - Overwrites if duplicate ID
    pub fn add_todo(self, description: String) -> Result<(Self, Vec<TodoEvent>), TodoError> {
        // ...
    }

    /// Updates an existing Todo with partial data
    /// 
    /// # Parameters
    /// - `self`: Takes ownership of Todos (immutable pattern)
    /// - `todo_update`: Partial update data with required `id` field
    /// 
    /// # Returns
    /// - `Ok((Todos, Vec<TodoEvent>))`: Returns updated Todos and events
    ///   - `[TodoEvent::TodoStateChanged]` if state is updated
    ///   - `[]` if only non-state fields are updated
    /// - `Err(TodoError::TodoNotFound)`: If `todo_update.id` doesn't exist in collection
    /// 
    /// # Special Requirements
    /// - Merges updates with existing Todo
    /// - Preserves all other Todos (immutable)
    /// - Requires `todo_update.id` to exist in collection
    /// - Triggers state change event if state is updated
    pub fn update_todo(self, todo_update: TodoUpdate) -> Result<(Self, Vec<TodoEvent>), TodoError> {
        // ...
    }

    /// Gets a Todo by ID
    /// 
    /// # Parameters
    /// - `self`: Reference to Todos
    /// - `id`: Todo identifier
    /// 
    /// # Returns
    /// - `Option<&Todo>`: Reference to Todo if exists, `None` otherwise
    pub fn get(&self, id: &str) -> Option<&Todo> {
        // ...
    }

    /// Gets a mutable Todo by ID
    /// 
    /// # Parameters
    /// - `self`: Mutable reference to Todos
    /// - `id`: Todo identifier
    /// 
    /// # Returns
    /// - `Option<&mut Todo>`: Mutable reference to Todo if exists, `None` otherwise
    pub fn get_mut(&mut self, id: &str) -> Option<&mut Todo> {
        // ...
    }

    /// Returns all Todo IDs
    /// 
    /// # Parameters
    /// - `self`: Reference to Todos
    /// 
    /// # Returns
    /// - `Vec<&String>`: Vector of all Todo ID references
    pub fn ids(&self) -> Vec<&String> {
        // ...
    }

    /// Returns the number of Todos
    /// 
    /// # Parameters
    /// - `self`: Reference to Todos
    /// 
    /// # Returns
    /// - `usize`: Number of Todos in the collection
    pub fn len(&self) -> usize {
        // ...
    }
}
```

#### Invariants

1. **Unique IDs**: Each Todo in the collection must have a unique ID (enforced by HashMap key)
2. **Immutable Operations**: Collection operations return new instances, preserving immutability
3. **Event Propagation**: Collection methods return events from underlying Todo operations

---

---

### Todo Repository

The `TodoRepository` trait defines the interface for persisting and retrieving Todo aggregates. This trait belongs to the domain layer and is implemented in the infrastructure layer, following the Dependency Inversion Principle.

#### Trait Definition

```rust
#[async_trait]
pub trait TodoRepository: Send + Sync {
    /// Saves a Todo aggregate to persistent storage
    /// 
    /// # Parameters
    /// - `todo`: The Todo aggregate to save
    /// 
    /// # Returns
    /// - `Ok(())`: Successfully saved
    /// - `Err(TodoError)`: If save operation fails
    /// 
    /// # Special Requirements
    /// - Handles both insert (new) and update (existing) operations
    /// - Persists all Todo fields including state
    async fn save(&self, todo: &Todo) -> Result<(), TodoError>;

    /// Finds a Todo by its unique identifier
    /// 
    /// # Parameters
    /// - `id`: The Todo identifier to search for
    /// 
    /// # Returns
    /// - `Ok(Option<Todo>)`: Returns `Some(Todo)` if found, `None` if not found
    /// - `Err(TodoError)`: If retrieval operation fails
    async fn find_by_id(&self, id: &str) -> Result<Option<Todo>, TodoError>;

    /// Finds all Todos in the repository
    /// 
    /// # Returns
    /// - `Ok(Vec<Todo>)`: Returns all Todos, empty vector if none exist
    /// - `Err(TodoError)`: If retrieval operation fails
    async fn find_all(&self) -> Result<Vec<Todo>, TodoError>;

    /// Deletes a Todo by its unique identifier
    /// 
    /// # Parameters
    /// - `id`: The Todo identifier to delete
    /// 
    /// # Returns
    /// - `Ok(())`: Successfully deleted (or Todo didn't exist)
    /// - `Err(TodoError)`: If deletion operation fails
    async fn delete(&self, id: &str) -> Result<(), TodoError>;
}
```

#### Implementation

The repository trait is implemented in the infrastructure layer (e.g., `sql_todo.repository.rs`) with concrete database operations. The implementation handles:
- Database connection management
- SQL query construction and execution
- Mapping between database records and domain entities
- Transaction management

---


## Error Types

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TodoError {
    EmptyDescription,
    InvalidStateTransition,
    TodoNotFound,
}
```

## Usage Examples

### Creating a Todo
```rust
let (todo, events) = Todo::new("Complete project documentation".to_string())?;
// todo: Todo { id: "uuid", state: TodoState::Todo, created_at: DateTime, description: "..." }
// events: [TodoEvent::TodoCreated { id, description, created_at }]

// Process events immediately
for event in events {
    // Handle TodoCreated event
}
```

### State Transitions
```rust
let (in_progress, events1) = todo.change_to_next_state()?;
// in_progress: Todo { ...todo, state: TodoState::InProgress, dirty: Some(true) }
// events1: [TodoEvent::TodoStateChanged { id, from_state: Todo, to_state: InProgress, changed_at }]

let (done, events2) = in_progress.change_to_next_state()?;
// done: Todo { ...in_progress, state: TodoState::Done, dirty: Some(true) }
// events2: [TodoEvent::TodoStateChanged { id, from_state: InProgress, to_state: Done, changed_at }]

// Process all events
for event in events1.into_iter().chain(events2) {
    // Handle state change events
}
```

### Updating State Directly
```rust
let (updated, events) = todo.update_state(TodoState::InProgress)?;
// updated: Todo { ...todo, state: TodoState::InProgress, dirty: Some(true) }
// events: [TodoEvent::TodoStateChanged { ... }]
```

### Checking State Transitions
```rust
if todo.is_new_state_allowed(TodoState::InProgress) {
    let (updated, events) = todo.update_state(TodoState::InProgress)?;
    // Process events
}

if TodoState::Todo.can_advance() {
    // Can transition to next state
}
```

### Collection Operations
```rust
// Create empty Todos collection
let mut todos = Todos::new();

// Add a new Todo - returns Todos and events
let (todos, mut all_events) = todos.add_todo(todo);
// todos: Todos { items: { todo.id => todo } }
// all_events: [TodoEvent::TodoCreated { ... }]

// Update a Todo using Todo methods
if let Some(existing_todo) = todos.get(&todo.id) {
    let (modified_todo, events) = existing_todo.change_to_next_state()?;
    all_events.extend(events);
    // Replace the todo in the collection
    let mut items = todos.items;
    items.insert(modified_todo.id.clone(), modified_todo);
    todos = Todos { items };
}

// Or use Todos method for partial updates
let (todos, events) = todos.update_todo(TodoUpdate {
    id: todo.id.clone(),
    state: Some(TodoState::Done),
    description: None,
})?;
all_events.extend(events);
// todos: Todos with updated Todo
// events: [TodoEvent::TodoStateChanged { ... }]

// Process all collected events
for event in all_events {
    // Handle events
}
```

### Partial Update Type

```rust
pub struct TodoUpdate {
    pub id: String,
    pub state: Option<TodoState>,
    pub description: Option<String>,
}
```

### Domain Events Usage

```rust
// Create a todo - returns Todo and TodoCreated event
let (todo, mut all_events) = Todo::new("Complete project documentation".to_string())?;
// all_events: [TodoEvent::TodoCreated { ... }]

// Change state - returns updated Todo and TodoStateChanged event
let (todo, events) = todo.change_to_next_state()?;
all_events.extend(events);
// all_events now contains: [TodoEvent::TodoCreated { ... }, TodoEvent::TodoStateChanged { ... }]

// Process all events after operations complete (e.g., update read models, send notifications)
for event in all_events {
    match event {
        TodoEvent::TodoCreated { id, description, .. } => {
            // Handle creation event
        }
        TodoEvent::TodoStateChanged { id, from_state, to_state, .. } => {
            // Handle state change event
        }
    }
}

// Events are handled immediately, no accumulation in the struct
```

