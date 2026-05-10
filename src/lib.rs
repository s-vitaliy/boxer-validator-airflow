use pyo3::exceptions::PyKeyError;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::wrap_pyfunction;
use std::collections::HashMap;

const PRINCIPAL_KEY: &str = "boxer.sneaksanddata.com/principal";
const USER_ID_KEY: &str = "boxer.sneaksanddata.com/external-identity";
const IDENTITY_PROVIDER_KEY: &str = "boxer.sneaksanddata.com/identity-provider";

#[pyclass]
struct BoxerPrincipal {
    principal: String,
    external_identity: String,
    identity_provider: String,
}

#[pymethods]
impl BoxerPrincipal {
    fn get_id(&self) -> String {
        self.boxer_user_id()
    }

    fn get_name(&self) -> String {
        self.boxer_user_id()
    }

    fn serialize_user(&self) -> HashMap<String, String> {
        HashMap::from([
            (PRINCIPAL_KEY.to_string(), self.principal.clone()),
            (USER_ID_KEY.to_string(), self.external_identity.clone()),
            (
                IDENTITY_PROVIDER_KEY.to_string(),
                self.identity_provider.clone(),
            ),
        ])
    }

    #[staticmethod]
    fn deserialize_user(token: &Bound<'_, PyDict>) -> PyResult<Self> {
        let principal = required_claim(token, PRINCIPAL_KEY)?;
        let external_identity = required_claim(token, USER_ID_KEY)?;
        let identity_provider = required_claim(token, IDENTITY_PROVIDER_KEY)?;

        Ok(Self {
            principal,
            external_identity,
            identity_provider,
        })
    }
}

impl BoxerPrincipal {
    fn boxer_user_id(&self) -> String {
        format!("{}/{}", self.identity_provider, self.external_identity)
    }
}

fn required_claim(token: &Bound<'_, PyDict>, key: &'static str) -> PyResult<String> {
    token
        .get_item(key)?
        .ok_or_else(|| PyKeyError::new_err(key))?
        .extract()
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

#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<BoxerPrincipal>()?;
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

#[cfg(test)]
mod tests {
    use super::BoxerPrincipal;

    #[test]
    fn serializes_user() {
        let user = BoxerPrincipal {
            principal: "principal-1".to_string(),
            external_identity: "u1".to_string(),
            identity_provider: "idp".to_string(),
        }
        .serialize_user();

        assert_eq!(
            user.get("boxer.sneaksanddata.com/principal")
                .map(String::as_str),
            Some("principal-1")
        );
        assert_eq!(
            user.get("boxer.sneaksanddata.com/external-identity")
                .map(String::as_str),
            Some("u1")
        );
        assert_eq!(
            user.get("boxer.sneaksanddata.com/identity-provider")
                .map(String::as_str),
            Some("idp")
        );
    }

    #[test]
    fn formats_user_id_from_identity_provider_and_external_identity() {
        let user = BoxerPrincipal {
            principal: "principal-1".to_string(),
            external_identity: "u1".to_string(),
            identity_provider: "idp".to_string(),
        };

        assert_eq!(user.boxer_user_id(), "idp/u1");
    }
}
