# Todo PyO3 Integration Guide

This document describes the file tree structure for the Todo Rust library and its Python bindings, and explains how to connect them using PyO3 and maturin.

## Overview

The Todo project uses PyO3 to create Python bindings for the Rust library, and maturin to build and package the Python library. This allows Python developers to use the high-performance Rust implementation while maintaining a Pythonic API.

**Key Setup Options:**
- **With `build.rs`**: Use `pyo3-build-config` in a `build.rs` script to enable `cargo build` for the entire workspace
- **Without `build.rs`**: Exclude `py-todo` from workspace and build only with `maturin` for Python distribution

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
├── build.rs                      # Build script for Python linking (uses pyo3-build-config)
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
edition = "2024"

[lib]
name = "todo"
crate-type = ["rlib", "staticlib"]

[dependencies]
chrono = { workspace = true }
async-trait = { workspace = true }
uuid = { workspace = true }
pyo3 = { workspace = true, optional = true }

[features]
default = []
python = ["pyo3"]
```

**Key points:**
- `[lib] crate-type = ["rlib", "staticlib"]` is required for the crate to be used as a library dependency by `py-todo`
- `[lib] name = "todo"` explicitly sets the library name (optional but recommended for clarity)
- PyO3 is optional so the core `todo` crate can be used without Python dependencies when the `python` feature is not enabled
- The `extension-module` feature is handled at the workspace level, not in individual crates

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
edition = "2024"

[lib]
name = "py_todo"
crate-type = ["cdylib", "rlib"]

[dependencies]
todo = { path = "../todo", features = ["python"] }
pyo3 = { workspace = true }

[build-dependencies]
pyo3-build-config = "0.27.2"

[features]
default = []
cython-compat = []
extension-module = [
    "pyo3/extension-module",
]
```

**Key points:**
- `name = "py_todo"` is the Rust crate name (underscores, not hyphens)
- `[lib] name = "py_todo"` must be unique to avoid library name collision with the `todo` crate
  - **Important**: The library name must differ from the `todo` crate's library name to prevent build errors
  - The Python module name is controlled by the `#[pymodule]` function name in `lib.rs`, not the library name
- `crate-type = ["cdylib", "rlib"]` is required for PyO3 extension modules
- `todo = { path = "../todo", features = ["python"] }` enables Python bindings in the todo crate
- `pyo3 = { workspace = true }` uses the workspace dependency (which includes `extension-module` feature)
- `[build-dependencies] pyo3-build-config` is required for the `build.rs` script to handle Python linking
- `extension-module` feature enables the PyO3 extension module functionality

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
fn py_todo(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Delegate to the todo::python module
    todo::python::todo(py, m)
}
```

**Key points:**
- The `#[pymodule]` function name (`py_todo`) determines the Python module name (`import py_todo`)
- If you want the Python module to be named `todo`, change the function name to `todo`:
  ```rust
  #[pymodule]
  fn todo(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
      todo::python::todo(py, m)
  }
  ```
- The library name (`py_todo`) is separate and only affects Rust's internal naming
- The function signature includes `Python<'_>` as the first parameter (required in PyO3 0.27+)
- This delegates to `todo::python::todo` which contains the actual bindings and registers all the classes

### Step 7: Create build.rs for py-todo

Create `crates/py-todo/build.rs` to handle Python linking:

```rust
fn main() {
    pyo3_build_config::add_extension_module_link_args();
}
```

This allows `py-todo` to be built with `cargo build` by properly linking against Python libraries.

### Step 8: Update Workspace Cargo.toml

You have two options for workspace configuration:

If you've created `build.rs` with `pyo3-build-config`, you can include `py-todo` in the workspace:

```toml
[workspace]
resolver = "3"
members = [
    "crates/todo",
    "crates/py-todo",
]

[workspace.dependencies]
chrono = { version = "0.4", features = ["serde"] }
async-trait = "0.1"
uuid = { version = "1.0", features = ["v4"] }
pyo3 = { version = "0.27.2", features = ["extension-module"] }
pyo3-build-config = "0.27.2"
```

**Benefits:**
- `cargo build` works for the entire workspace
- Better IDE support and tooling integration
- Consistent dependency management

**Requirements:**
- Must have `build.rs` with `pyo3_build_config::add_extension_module_link_args()`
- Must add `pyo3-build-config` to workspace dependencies
- Must add `pyo3-build-config` as a build-dependency in `py-todo/Cargo.toml`

**When to use this:**
- If you prefer to only build `py-todo` with `maturin`
- If you want to avoid Python linking during regular `cargo build`
- The `todo` crate can still be built normally with `cargo build -p todo` or just `cargo build` (since `py-todo` is excluded)

### Step 9: Install Maturin

```bash
pipx install maturin
# or
uv tool install maturin
# or
pip install maturin
# or
cargo install maturin
```

### Step 10: Build Python Package

From `crates/py-todo/` directory:

```bash
# Development build (editable install)
maturin develop

# Build wheel
maturin build

# Build and publish
maturin publish
```

### Step 11: Use in Python

**Note**: The Python module name is determined by the `#[pymodule]` function name in `crates/py-todo/src/lib.rs`. If it's named `py_todo`, use `import py_todo`. If you want it to be `todo`, change the function name to `todo`.

```python
import py_todo  # or 'import todo' if you changed the module name

# Create a new todo (returns only the todo object)
todo_obj = py_todo.PyTodo("Buy groceries")

# Or create with events
todo_obj, events = py_todo.PyTodo.create("Buy groceries")

# Change state
events = todo_obj.update_state(py_todo.PyTodoState.IN_PROGRESS)

# Access properties
print(todo_obj.id)
print(todo_obj.description)
print(todo_obj.state)
print(todo_obj.created_at)
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

**Important**: The `py-todo` crate can be built in two ways:

#### Option 1: Using Maturin (Recommended for Python Distribution)

For creating Python wheels and distributions, use `maturin`:

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

**Why use maturin?**
- Handles Python detection, linking, and packaging automatically
- Creates proper Python wheels (`.whl` files) and source distributions
- Manages Python package metadata correctly
- Required for publishing to PyPI

#### Option 2: Using Cargo Build (With build.rs)

If you've set up `build.rs` with `pyo3-build-config`, you can also build with `cargo build`:

```bash
# From workspace root
cargo build -p py_todo

# Or if py-todo is in workspace members
cargo build
```

**When to use cargo build:**
- For development and testing
- When you want to use standard Rust tooling
- When you don't need Python wheels

**Requirements for cargo build:**
- Must have `build.rs` with `pyo3_build_config::add_extension_module_link_args()`
- Must have `pyo3-build-config` in `[build-dependencies]`
- The resulting library can be used with Python, but won't be a proper Python package

## Troubleshooting

### Build Error: "Undefined symbols for architecture"

**Symptom**: `cargo build` fails with errors like:
```
Undefined symbols for architecture arm64:
  "_PyBaseObject_Type", referenced from: ...
```

**Cause**: Trying to build `py-todo` with `cargo build` without proper Python linking setup.

**Solution**: 
1. **If using Option A (include in workspace)**: Ensure you have:
   - `build.rs` file with `pyo3_build_config::add_extension_module_link_args()`
   - `pyo3-build-config` in `[build-dependencies]` in `py-todo/Cargo.toml`
   - `pyo3-build-config` in workspace dependencies
2. **If using Option B (exclude from workspace)**: 
   - Ensure `py-todo` is excluded from workspace members (see Step 8)
   - Use `maturin develop` or `maturin build` to build the Python extension
   - Use `cargo build -p todo` to build only the Rust library

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
1. Verify `exclude = ["crates/py-todo"]` is in the root `Cargo.toml` (if using Option B)
2. Run `cargo clean` to clear cached build state
3. Use `cargo build -p todo` to explicitly build only the `todo` crate

### Build Error: Missing `pyo3-build-config`

**Symptom**: `cargo build` fails with errors about missing `pyo3_build_config` module.

**Cause**: `build.rs` uses `pyo3-build-config` but it's not in build-dependencies.

**Solution**:
1. Add `pyo3-build-config = "0.27.2"` to `[build-dependencies]` in `crates/py-todo/Cargo.toml`
2. Ensure `pyo3-build-config` is in workspace dependencies in root `Cargo.toml`

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

