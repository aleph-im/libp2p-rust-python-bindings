use pyo3::prelude::*;

pub(crate) fn register(_py: Python<'_>, _m: &PyModule) -> PyResult<()> {
    // m.add_class::<SomeClass>()?;
    Ok(())
}
