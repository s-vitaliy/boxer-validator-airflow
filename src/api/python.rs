use std::collections::HashMap;

use pyo3::exceptions::PyKeyError;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::wrap_pyfunction;

use crate::models::boxer_principal::BoxerPrincipal;

#[pyclass(name = "BoxerPrincipal")]
struct PythonBoxerPrincipal {
    inner: BoxerPrincipal,
}

#[pymethods]
impl PythonBoxerPrincipal {
    fn get_id(&self) -> String {
        self.inner.get_id()
    }

    fn get_name(&self) -> String {
        self.inner.get_name()
    }

    fn serialize_user(&self) -> HashMap<String, String> {
        self.inner.serialize_user()
    }

    #[staticmethod]
    fn deserialize_user(token: HashMap<String, String>) -> PyResult<Self> {
        let inner = BoxerPrincipal::deserialize_user(&token)
            .map_err(|error| PyKeyError::new_err(error.key()))?;

        Ok(Self { inner })
    }
}

#[pyfunction]
fn get_url_login(_kwargs: &Bound<'_, PyDict>) -> String {
    String::new()
}

#[pyfunction]
fn filter_authorized_menu_items(_menu_items: Vec<Py<PyAny>>, _user_id: String) -> Vec<Py<PyAny>> {
    Vec::new()
}

#[pyfunction]
fn is_authorized_asset(_method: String, _user_id: String, _details: Option<Py<PyAny>>) -> bool {
    false
}

#[pyfunction]
fn is_authorized_asset_alias(
    _method: String,
    _user_id: String,
    _details: Option<Py<PyAny>>,
) -> bool {
    false
}

#[pyfunction]
fn is_authorized_configuration(
    _method: String,
    _user_id: String,
    _details: Option<Py<PyAny>>,
) -> bool {
    false
}

#[pyfunction]
fn is_authorized_connection(
    _method: String,
    _user_id: String,
    _details: Option<Py<PyAny>>,
) -> bool {
    false
}

#[pyfunction]
fn is_authorized_custom_view(_method: String, _resource_name: String, _user_id: String) -> bool {
    false
}

#[pyfunction]
fn is_authorized_dag(
    _method: String,
    _user_id: String,
    _access_entity: Option<Py<PyAny>>,
    _details: Option<Py<PyAny>>,
) -> bool {
    false
}

#[pyfunction]
fn is_authorized_pool(_method: String, _user_id: String, _details: Option<Py<PyAny>>) -> bool {
    false
}

#[pyfunction]
fn is_authorized_variable(_method: String, _user_id: String, _details: Option<Py<PyAny>>) -> bool {
    false
}

#[pyfunction]
fn is_authorized_view(_access_view: Py<PyAny>, _user_id: String) -> bool {
    false
}

pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PythonBoxerPrincipal>()?;
    m.add_function(wrap_pyfunction!(get_url_login, m)?)?;
    m.add_function(wrap_pyfunction!(filter_authorized_menu_items, m)?)?;
    m.add_function(wrap_pyfunction!(is_authorized_asset, m)?)?;
    m.add_function(wrap_pyfunction!(is_authorized_asset_alias, m)?)?;
    m.add_function(wrap_pyfunction!(is_authorized_configuration, m)?)?;
    m.add_function(wrap_pyfunction!(is_authorized_connection, m)?)?;
    m.add_function(wrap_pyfunction!(is_authorized_custom_view, m)?)?;
    m.add_function(wrap_pyfunction!(is_authorized_dag, m)?)?;
    m.add_function(wrap_pyfunction!(is_authorized_pool, m)?)?;
    m.add_function(wrap_pyfunction!(is_authorized_variable, m)?)?;
    m.add_function(wrap_pyfunction!(is_authorized_view, m)?)?;
    Ok(())
}
