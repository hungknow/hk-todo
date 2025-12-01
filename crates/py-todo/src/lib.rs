use pyo3::prelude::*;

// Re-export the Python module from todo crate
// The module is already initialized by todo::python::todo
// This crate just serves as the entry point for maturin
#[pymodule]
fn py_todo(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Delegate to the todo::python module
    todo::python::todo(py, m)
}

