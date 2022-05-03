use pyo3::prelude::*;
use libp2p::{PeerId};
use crate::core::identity::{PyPublicKey};

#[pyclass(name = "PeerId")]
struct PyPeerId {
    peer_id: PeerId,
}

#[pymethods]
impl PyPeerId {
    #[new]
    fn py_new(public_key: PyPublicKey) -> PyResult<Self> {
        Ok(PyPeerId { peer_id: PeerId::from(public_key.public_key) })
    }

    fn __repr__(&self) -> String {
        format!("{}", self.peer_id)
    }

    fn __str__(&self) -> String {
        self.__repr__()
    }
}

// #[pymethods]
// impl PyPublicKey {
//
// }

pub(crate) fn register(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyPeerId>()?;

    Ok(())
}
