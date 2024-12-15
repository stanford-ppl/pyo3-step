use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn create_context() -> PyResult<String> {
    // Create a context and return it
    todo!()
}

#[pyfunction]
fn add_node(// PyResult<String> // Context
// Information for the node to add
) -> PyResult<String> {
    todo!()
}

#[pyfunction]
fn run_simulation(// PyResult<String> // Context
    // features for running the simulation
) -> PyResult<String> {
    todo!()
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_step(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
