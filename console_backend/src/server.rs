//! Objects related to PyBuffer and PyStr
use pyo3::prelude::*;
use pyo3::types::PyBytes;

/// The backend server
#[pyclass]
struct Server {}

#[pymethods]
impl Server {
    #[new]
    pub fn __new__() -> Self {
        Server {}
    }

    #[text_signature = "($self, bytes, /)"]
    pub fn from_bytes(&mut self, bytes: &PyBytes) -> PyResult<usize> {
        println!("Hello");
        let byte_vec: Vec<u8> = bytes.extract().unwrap();
        Ok(byte_vec.len())
    }
}

#[pymodule]
pub fn server(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Server>()?;
    Ok(())
}
