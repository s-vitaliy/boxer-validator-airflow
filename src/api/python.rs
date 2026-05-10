use std::collections::HashMap;

use pyo3::exceptions::PyKeyError;
use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::models::boxer_principal::BoxerPrincipal;
use crate::services::validation_service::Boxer;

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

#[pyclass(name = "Boxer")]
struct PythonBoxer {
    inner: Boxer,
}

#[pymethods]
impl PythonBoxer {
    #[new]
    fn new() -> Self {
        Self { inner: Boxer::new() }
    }

    fn get_url_login(&self, _kwargs: &Bound<'_, PyDict>) -> String {
        self.inner.get_url_login()
    }

    fn filter_authorized_menu_items(
        &self,
        menu_items: Vec<Py<PyAny>>,
        user_id: String,
    ) -> Vec<Py<PyAny>> {
        self.inner.filter_authorized_menu_items(menu_items, &user_id)
    }

    fn is_authorized_asset(
        &self,
        method: String,
        user_id: String,
        _details: Option<Py<PyAny>>,
    ) -> bool {
        self.inner.is_authorized_asset(&method, &user_id)
    }

    fn is_authorized_asset_alias(
        &self,
        method: String,
        user_id: String,
        _details: Option<Py<PyAny>>,
    ) -> bool {
        self.inner.is_authorized_asset_alias(&method, &user_id)
    }

    fn is_authorized_configuration(
        &self,
        method: String,
        user_id: String,
        _details: Option<Py<PyAny>>,
    ) -> bool {
        self.inner.is_authorized_configuration(&method, &user_id)
    }

    fn is_authorized_connection(
        &self,
        method: String,
        user_id: String,
        _details: Option<Py<PyAny>>,
    ) -> bool {
        self.inner.is_authorized_connection(&method, &user_id)
    }

    fn is_authorized_custom_view(
        &self,
        method: String,
        resource_name: String,
        user_id: String,
    ) -> bool {
        self.inner.is_authorized_custom_view(&method, &resource_name, &user_id)
    }

    fn is_authorized_dag(
        &self,
        method: String,
        user_id: String,
        _access_entity: Option<Py<PyAny>>,
        _details: Option<Py<PyAny>>,
    ) -> bool {
        self.inner.is_authorized_dag(&method, &user_id)
    }

    fn is_authorized_pool(
        &self,
        method: String,
        user_id: String,
        _details: Option<Py<PyAny>>,
    ) -> bool {
        self.inner.is_authorized_pool(&method, &user_id)
    }

    fn is_authorized_variable(
        &self,
        method: String,
        user_id: String,
        _details: Option<Py<PyAny>>,
    ) -> bool {
        self.inner.is_authorized_variable(&method, &user_id)
    }

    fn is_authorized_view(&self, access_view: Py<PyAny>, user_id: String) -> bool {
        let _ = access_view;
        self.inner.is_authorized_view("", &user_id)
    }
}

pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PythonBoxerPrincipal>()?;
    m.add_class::<PythonBoxer>()?;
    Ok(())
}

