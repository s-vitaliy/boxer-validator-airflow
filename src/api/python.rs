use std::collections::HashMap;
use std::env;
use std::str::FromStr;
use std::sync::Arc;

use cedar_policy::PolicySet;
use pyo3::exceptions::{PyKeyError, PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use tokio::runtime::Handle;

use crate::models::boxer_principal::BoxerPrincipal;
use crate::models::entities::{AirflowEntity, action_entity_uid};
use crate::services::static_policy_repository::StaticPolicyRepository;
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

const POLICY_SET_ENV_VAR: &str = "PYTHON_BOXER_POLICY_SET";

#[pyclass(name = "Boxer")]
struct PythonBoxer {
    inner: Boxer,
}

#[pymethods]
impl PythonBoxer {
    #[new]
    fn new() -> PyResult<Self> {
        let policies = env::var(POLICY_SET_ENV_VAR)
            .map_err(|_| PyRuntimeError::new_err(format!("environment variable {POLICY_SET_ENV_VAR} is not set")))?;
        let policy_set = policies
            .parse::<PolicySet>()
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        let repository = Arc::new(StaticPolicyRepository::new(policy_set));
        Ok(Self {
            inner: Boxer::new(repository),
        })
    }

    fn get_url_login(&self, _kwargs: &Bound<'_, PyDict>) -> String {
        self.inner.get_url_login()
    }

    fn filter_authorized_menu_items(
        &self,
        menu_items: Vec<Py<PyAny>>,
        user_id: String,
    ) -> Vec<Py<PyAny>> {
        self.inner
            .filter_authorized_menu_items(menu_items, &user_id)
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

        let result = Handle::current().block_on(self.inner.is_authorized(user, action, resource));

        Ok(result)
    }
}

pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PythonBoxerPrincipal>()?;
    m.add_class::<PythonBoxer>()?;
    Ok(())
}
