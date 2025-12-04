#[cfg(feature = "dart")]
mod frb_generated; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */

pub mod domain;
pub mod infrastructure;
pub mod application;

// Ensure features are mutually exclusive at compile time
#[cfg(all(feature = "python", feature = "dart"))]
compile_error!("Features 'python' and 'dart' are mutually exclusive. Please enable only one.");

#[cfg(feature = "python")]
pub mod python;

#[cfg(feature = "dart")]
pub mod dart;

// Re-export commonly used domain types for convenience
pub use domain::todo::{
    Todo, TodoError, TodoEvent, TodoRepository, TodoState,
};
