use libp2p::core::PublicKey;
use pyo3::prelude::*;
use libp2p::identity;

#[pyclass]
struct PyKeypair {
    keypair: identity::Keypair,
}

#[pyclass]
struct PyPublicKey {
    public_key: PublicKey,
}

#[pymethods]
impl PyKeypair {
    #[staticmethod]
    fn generate_ed25519() -> PyResult<Self> {
        let keypair = identity::Keypair::generate_ed25519();
        Ok(PyKeypair { keypair })
    }

    fn public(&self) -> PyResult<PyPublicKey> {
        Ok(PyPublicKey { public_key: self.keypair.public() })
    }
}

// #[pymethods]
// impl PyPublicKey {
//
// }

pub(crate) fn register(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    let identity_module = PyModule::new(py, "identity")?;
    identity_module.add_class::<PyKeypair>()?;
    identity_module.add_class::<PyPublicKey>()?;

    m.add_submodule(identity_module)?;
    Ok(())
}
