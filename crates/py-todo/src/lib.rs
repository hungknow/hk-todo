use pyo3::prelude::*;

// Re-export the Python module from todo crate
// The module is already initialized by todo::python::todo
// This crate just serves as the entry point for maturin
#[pymodule]
fn todo(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Delegate to the todo::python module
    // Use ::todo to refer to the crate unambiguously
    ::todo::python::todo(m)
}

