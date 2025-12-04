use crate::domain::todo::{Todo, TodoError};

#[cfg(feature = "dart")]
use flutter_rust_bridge::frb;

/// Repository trait for persisting and retrieving Todo aggregates
/// 
/// This trait belongs to the domain layer and is implemented in the infrastructure layer,
/// following the Dependency Inversion Principle.
#[cfg_attr(feature = "dart", frb)]
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
    fn save(&self, todo: &Todo) -> Result<(), TodoError>;

    /// Finds a Todo by its unique identifier
    /// 
    /// # Parameters
    /// - `id`: The Todo identifier to search for
    /// 
    /// # Returns
    /// - `Ok(Option<Todo>)`: Returns `Some(Todo)` if found, `None` if not found
    /// - `Err(TodoError)`: If retrieval operation fails
    fn find_by_id(&self, id: &str) -> Result<Option<Todo>, TodoError>;

    /// Finds all Todos in the repository
    /// 
    /// # Returns
    /// - `Ok(Vec<Todo>)`: Returns all Todos, empty vector if none exist
    /// - `Err(TodoError)`: If retrieval operation fails
    fn find_all(&self) -> Result<Vec<Todo>, TodoError>;

    /// Deletes a Todo by its unique identifier
    /// 
    /// # Parameters
    /// - `id`: The Todo identifier to delete
    /// 
    /// # Returns
    /// - `Ok(())`: Successfully deleted (or Todo didn't exist)
    /// - `Err(TodoError)`: If deletion operation fails
    fn delete(&self, id: &str) -> Result<(), TodoError>;
}
