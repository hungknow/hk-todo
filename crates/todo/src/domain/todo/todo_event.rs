use chrono::{DateTime, Utc};
use crate::domain::todo::TodoState;

#[cfg(feature = "dart")]
use flutter_rust_bridge::frb;

/// Domain events that describe significant occurrences in the Todo lifecycle
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dart", frb)]
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

