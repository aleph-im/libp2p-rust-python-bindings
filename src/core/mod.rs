pub mod identity;
pub mod peer_id;

use pyo3::prelude::*;

pub(crate) fn register(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    identity::register(py, m)?;
    peer_id::register(py, m)?;
    Ok(())
}
