# Todo PyO3 Integration Guide

This document describes the file tree structure for the Todo Rust library and its Python bindings, and explains how to connect them using PyO3 and maturin.

## Overview

The Todo project uses PyO3 to create Python bindings for the Rust library, and maturin to build and package the Python library. This allows Python developers to use the high-performance Rust implementation while maintaining a Pythonic API.

## File Tree Structure

### Rust Library Structure

```
crates/todo/
├── Cargo.toml                    # Rust package configuration
├── src/
│   ├── lib.rs                    # Library root, re-exports domain types
│   ├── domain/
│   │   ├── mod.rs
│   │   └── todo/
│   │       ├── mod.rs            # Domain module exports
│   │       ├── todo_entity.rs    # Todo aggregate root
│   │       ├── todo_state.rs     # TodoState enum (Todo, InProgress, Done)
│   │       ├── todo_error.rs     # TodoError enum
│   │       ├── todo_event.rs     # TodoEvent enum
│   │       └── todo_repository.rs # TodoRepository trait
│   ├── application/
│   │   ├── mod.rs
│   │   ├── add_todo_handler.rs   # Application handler for creating todos
│   │   ├── get_todos_handler.rs  # Application handler for retrieving todos
│   │   └── change_todo_state_handler.rs # Application handler for state changes
│   └── infrastructure/
│       ├── mod.rs
│       └── repositories/
│           └── todo/
│               ├── mod.rs
│               └── inmemory_todo_repository.rs # In-memory repository implementation
├── python/                       # PyO3 bindings module
│   └── mod.rs                    # Python bindings that wrap todo domain types
└── tests/
    ├── add_todo.rs
    ├── change_todo_state.rs
    └── common/
        ├── mod.rs
        └── mock_todo_repository.rs
```

### Python Bindings Structure

```
crates/py-todo/
├── Cargo.toml                    # Root crate configuration (depends on todo crate)
├── pyproject.toml                # Python package configuration (maturin)
└── src/
    └── lib.rs                    # Re-exports Python bindings from todo::python
```

### Workspace Structure

```
todo-ddd/
├── Cargo.toml                    # Workspace configuration (includes pyo3 dependency)
├── crates/
│   ├── todo/                     # Core Rust library (domain logic)
│   └── py-todo/                  # Python bindings crate (depends on todo)
└── docs/
    ├── todo-domain-model.md
    └── todo_pyo3.md              # This file
```

## How Rust and Python Connect

### 1. PyO3 Bindings Layer

PyO3 provides the bridge between Rust and Python. The bindings are typically defined in the Rust library's `lib.rs` or a dedicated Python module file.

**Key Components:**

- **`#[pyclass]`**: Marks Rust structs to be exposed as Python classes
- **`#[pymethods]`**: Marks methods to be exposed to Python
- **`#[pyfunction]`**: Marks functions to be exposed to Python
- **`#[derive(FromPyObject)]`**: Allows Python objects to be converted to Rust types
- **`Python::with_gil()`**: Manages Python's Global Interpreter Lock

### 2. Maturin Build System

Maturin is the build tool that:
- Compiles Rust code with PyO3 bindings
- Generates Python wheels (`.whl` files)
- Creates source distributions (`.tar.gz`)
- Handles Python package metadata

### 3. Integration Flow

```
┌─────────────────┐
│  Python Code    │
│  (import todo)  │
└────────┬────────┘
         │
         │ Python API calls
         ▼
┌─────────────────┐
│  py-todo crate  │  ← Root crate (crates/py-todo/src/lib.rs)
│  (Re-exports)   │     Re-exports todo::python module
└────────┬────────┘
         │
         │ Rust dependency
         ▼
┌─────────────────┐
│  todo::python   │  ← PyO3 bindings (crates/todo/python/mod.rs)
│  PyO3 Bindings  │     Wraps domain types for Python
│  (Rust ↔ Python)│
└────────┬────────┘
         │
         │ Direct access to domain
         ▼
┌─────────────────┐
│  todo crate     │  ← Core Rust library (crates/todo/src/)
│  (Domain Logic) │     Domain, Application, Infrastructure
└─────────────────┘
```

## Setup Instructions

### Step 1: Add PyO3 Dependencies to todo Crate

Update `crates/todo/Cargo.toml` to include PyO3 as an optional dependency:

```toml
[package]
name = "todo"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["rlib", "staticlib"]

[dependencies]
chrono = { workspace = true }
async-trait = { workspace = true }
uuid = { workspace = true }
pyo3 = { workspace = true, optional = true, features = ["extension-module"] }

[features]
default = []
python = ["pyo3"]
```

**Key points:**
- `[lib] crate-type = ["rlib", "staticlib"]` is required for the crate to be used as a library dependency by `py-todo`
- PyO3 is optional so the core `todo` crate can be used without Python dependencies when the `python` feature is not enabled

### Step 2: Create Python Bindings Module in todo Crate

Create `crates/todo/python/mod.rs` with PyO3 bindings:

```rust
use pyo3::prelude::*;
use pyo3::types::PyModule;
use crate::{Todo, TodoState, TodoError, TodoEvent};

// Python-compatible wrappers for domain types
#[pyclass]
pub struct PyTodo {
    inner: Todo,
}

#[pymethods]
impl PyTodo {
    #[new]
    fn new(description: String) -> PyResult<(Self, Vec<PyTodoEvent>)> {
        let (todo, events) = Todo::new(description)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(
                format!("Failed to create todo: {:?}", e)
            ))?;
        // ... convert events and return
    }
    
    fn update_state(&mut self, new_state: PyTodoState) -> PyResult<Vec<PyTodoEvent>> {
        // ... implementation using self.inner
    }
}

// Python module initialization
#[pymodule]
fn todo(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyTodo>()?;
    m.add_class::<PyTodoState>()?;
    // ... register other classes
    Ok(())
}
```

Update `crates/todo/src/lib.rs` to conditionally expose the python module:

```rust
pub mod domain;
pub mod infrastructure;
pub mod application;

#[cfg(feature = "python")]
pub mod python;

// Re-export commonly used domain types for convenience
pub use domain::todo::{
    Todo, TodoError, TodoEvent, TodoRepository, TodoState,
};
```

### Step 3: Create py-todo Root Crate

Create `crates/py-todo/` as a root crate that re-exports the Python bindings:

```
crates/py-todo/
├── Cargo.toml
├── pyproject.toml
└── src/
    └── lib.rs
```

### Step 4: Configure Cargo.toml for py-todo

The `crates/py-todo/Cargo.toml` should depend on the `todo` crate with the `python` feature:

```toml
[package]
name = "py_todo"
version = "0.1.0"
edition = "2021"

[lib]
name = "py_todo"
crate-type = ["cdylib", "rlib"]

[dependencies]
todo = { path = "../todo", features = ["python"] }
pyo3 = { workspace = true, features = ["extension-module"] }
```

**Key points:**
- `name = "py_todo"` is the Rust crate name (underscores, not hyphens)
- `[lib] name = "py_todo"` must be unique to avoid library name collision with the `todo` crate
  - **Important**: The library name must differ from the `todo` crate's library name to prevent build errors
  - The Python module name is controlled by the `#[pymodule]` function name in `lib.rs`, not the library name
- `crate-type = ["cdylib", "rlib"]` is required for PyO3 extension modules
- `todo = { path = "../todo", features = ["python"] }` enables Python bindings in the todo crate
- `pyo3 = { workspace = true, features = ["extension-module"] }` is required for the extension module

### Step 5: Create pyproject.toml

Create `pyproject.toml` in `crates/py-todo/`:

```toml
[build-system]
requires = ["maturin>=1.0,<2.0"]
build-backend = "maturin"

[project]
name = "todo"
requires-python = ">=3.8"
classifiers = [
    "Programming Language :: Rust",
    "Programming Language :: Python :: Implementation :: CPython",
    "Programming Language :: Python :: Implementation :: PyPy",
]

[tool.maturin]
features = []
compatibility = "linux"
```

### Step 6: Define Re-exports in py-todo

In `crates/py-todo/src/lib.rs`, re-export the Python module from `todo`:

```rust
use pyo3::prelude::*;

// Re-export the Python module from todo crate
// The module is already initialized by todo::python::todo
// This crate just serves as the entry point for maturin
#[pymodule]
fn todo(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Delegate to the todo::python module
    ::todo::python::todo(m)
}
```

**Key points:**
- The `#[pymodule]` function name (`todo`) determines the Python module name (`import todo`)
- The library name (`py_todo`) is separate and only affects Rust's internal naming
- This delegates to `todo::python::todo` which contains the actual bindings

### Step 7: Update Workspace Cargo.toml

**Important**: `py-todo` should be **excluded** from workspace members, not included. This is because:

1. PyO3 extension modules require Python linking, which `cargo build` doesn't handle automatically
2. `py-todo` should be built with `maturin`, not `cargo build`
3. Including it in workspace members causes `cargo build` to fail with linker errors

Configure the root `Cargo.toml` as follows:

```toml
[workspace]
resolver = "3"
members = [
    "crates/todo",
]
exclude = [
    "crates/py-todo",
]

[workspace.dependencies]
chrono = { version = "0.4", features = ["serde"] }
async-trait = "0.1"
uuid = { version = "1.0", features = ["v4"] }
pyo3 = "0.27.2"
```

**Why exclude `py-todo`?**
- `cargo build` will fail when trying to build `py-todo` because it requires Python library linking
- PyO3 extension modules must be built with `maturin`, which handles Python linking automatically
- The `todo` crate can still be built normally with `cargo build -p todo` or just `cargo build` (since `py-todo` is excluded)

### Step 8: Install Maturin

```bash
pip install maturin
# or
cargo install maturin
```

### Step 9: Build Python Package

From `crates/py-todo/` directory:

```bash
# Development build (editable install)
maturin develop

# Build wheel
maturin build

# Build and publish
maturin publish
```

### Step 10: Use in Python

```python
import todo

# Create a new todo
todo_obj, events = todo.Todo("Buy groceries")

# Change state
events = todo_obj.update_state(todo.TodoState.IN_PROGRESS)

# Access properties
print(todo_obj.id)
print(todo_obj.description)
print(todo_obj.state)
```

## Async Considerations

Since the Rust code uses `async` functions, you'll need to handle async in Python bindings:

- Use `pyo3-asyncio` for async support
- Or provide sync wrappers that use `Python::with_gil()` and block on futures
- Or expose async functions directly using `#[pyo3(async)]`

## Repository Pattern

The repository trait (`TodoRepository`) can be:
- Implemented in Python (using `#[pyclass]` and `#[pymethods]`)
- Or wrapped with a concrete implementation (e.g., `InMemoryTodoRepository`) exposed to Python

## Architecture Benefits

This architecture provides several advantages:

1. **Separation of Concerns**: 
   - PyO3 bindings live in `todo::python` module, keeping domain logic separate
   - The `python` feature flag allows using `todo` crate without Python dependencies
   - `py-todo` is a thin root crate that just re-exports bindings

2. **Clean Dependencies**: 
   - PyO3 is optional in the `todo` crate (via feature flag)
   - Only enabled when building Python bindings
   - Core library can be used in pure Rust projects

3. **Modularity**: 
   - Python bindings are co-located with the domain code they wrap
   - Easy to maintain and understand the mapping between Rust and Python types
   - Other language bindings can follow the same pattern (e.g., `todo::nodejs`, `todo::go`)

4. **Build Isolation**: 
   - `py-todo` handles maturin configuration and Python packaging
   - `todo` crate remains focused on domain logic
   - Clear separation between packaging and implementation

## Building

### Building the Rust Library

The core `todo` crate can be built normally:

```bash
# From workspace root
cargo build

# Or specify the package explicitly
cargo build -p todo

# Run tests
cargo test -p todo
```

### Building the Python Extension

**Important**: The `py-todo` crate must be built with `maturin`, not `cargo build`:

```bash
# From crates/py-todo directory
cd crates/py-todo

# Development build (editable install)
maturin develop

# Build wheel
maturin build

# Build and publish
maturin publish
```

**Why not `cargo build`?**
- PyO3 extension modules require linking against Python libraries
- `cargo build` doesn't automatically find and link Python
- `maturin` handles Python detection, linking, and packaging automatically

## Troubleshooting

### Build Error: "Undefined symbols for architecture"

**Symptom**: `cargo build` fails with errors like:
```
Undefined symbols for architecture arm64:
  "_PyBaseObject_Type", referenced from: ...
```

**Cause**: Trying to build `py-todo` with `cargo build` instead of `maturin`.

**Solution**: 
1. Ensure `py-todo` is excluded from workspace members (see Step 7)
2. Use `maturin develop` or `maturin build` to build the Python extension
3. Use `cargo build -p todo` to build only the Rust library

### Build Error: "output filename collision"

**Symptom**: Warning or error about colliding filenames:
```
warning: output filename collision.
The lib target `todo` in package `todo` has the same output filename as the lib target `todo` in package `py_todo`.
```

**Cause**: Both `todo` and `py-todo` crates have the same library name.

**Solution**: 
1. In `crates/py-todo/Cargo.toml`, set `[lib] name = "py_todo"` (not `"todo"`)
2. The Python module name is controlled by the `#[pymodule]` function name, not the library name

### `cargo build` Still Tries to Build `py-todo`

**Symptom**: Even after excluding it, `cargo build` still attempts to build `py-todo`.

**Solution**:
1. Verify `exclude = ["crates/py-todo"]` is in the root `Cargo.toml`
2. Run `cargo clean` to clear cached build state
3. Use `cargo build -p todo` to explicitly build only the `todo` crate

## Testing

Test Python bindings with:

```bash
# Run Rust tests for core library
cd crates/todo
cargo test

# Build and test Python bindings
cd crates/py-todo
maturin develop
python -m pytest tests/  # If you add Python tests
```

