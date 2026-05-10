mod api;
mod models;
mod services;

use pyo3::prelude::*;

#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    api::python::register(m)
}
