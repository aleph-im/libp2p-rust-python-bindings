use pyo3::prelude::*;
use libp2p::core::transport::Boxed;
use libp2p::PeerId;
use libp2p::core::muxing::StreamMuxerBox;

use crate::core::identity::{PyKeypair};

#[pyclass(name = "Transport")]
struct PyTransport {
    transport: Boxed<(PeerId, StreamMuxerBox)>,
}

#[pymethods]
impl PyTransport {}

#[pyfunction(name = "development_transport")]
fn py_development_transport(py: Python<'_>, keypair: PyKeypair) -> PyResult<&PyAny> {
    pyo3_asyncio::async_std::future_into_py(py, async {
        let transport = libp2p::development_transport(keypair.keypair).await?;
        Ok(Python::with_gil(|_py| PyTransport{transport}))
    })
}

pub(crate) fn register(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_development_transport, m)?)?;
    Ok(())
}
