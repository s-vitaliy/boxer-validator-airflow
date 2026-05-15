use std::collections::HashMap;
use std::str::FromStr;

use cedar_policy::PolicySet;
use pyo3::exceptions::{PyKeyError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::models::boxer_principal::BoxerPrincipal;
use crate::models::entities::{action_entity_uid, AirflowEntity};
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
    fn new(policies: String) -> PyResult<Self> {
        let policy_set = policies
            .parse::<PolicySet>()
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(Self { inner: Boxer::new(policy_set) })
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

    /// Check whether `user_id` is allowed to perform `method` on a resource of type `resource_type`.
    ///
    /// - `method`: HTTP-style verb — `"GET"`, `"POST"`, `"PUT"`, `"DELETE"`.
    ///   `"GET"` without `entity_id` is treated as `"LIST"`.
    /// - `resource_type`: Cedar entity kind name — `"Dag"`, `"Variable"`, `"Custom"`, etc.
    /// - `user_id`: opaque user identifier.
    /// - `entity_id`: specific resource ID (optional); determines GET vs LIST and scopes the resource UID.
    fn is_authorized(
        &self,
        method: String,
        resource_type: String,
        user_id: String,
        entity_id: Option<String>,
    ) -> PyResult<bool> {
        let resource_entity = AirflowEntity::from_str(&resource_type)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;

        let action = action_entity_uid(&method, entity_id.as_deref());
        let user = AirflowEntity::User.entity_uid(&user_id);
        let resource = resource_entity.entity_uid(entity_id.as_deref().unwrap_or(""));

        Ok(self.inner.is_authorized(action, user, resource))
    }
}

pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PythonBoxerPrincipal>()?;
    m.add_class::<PythonBoxer>()?;
    Ok(())
}

