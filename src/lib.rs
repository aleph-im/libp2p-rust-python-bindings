mod swarm;
mod transport;
mod core;

use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn libp2p_rust_python_bindings(py: Python, m: &PyModule) -> PyResult<()> {
    core::identity::register(py, m)?;
    swarm::register(py, m)?;
    transport::register(py, m)?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
