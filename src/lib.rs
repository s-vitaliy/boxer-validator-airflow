use pyo3::exceptions::PyKeyError;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::wrap_pyfunction;
use std::collections::HashMap;

#[pyclass]
struct BoxerPrincipal {
    user_id: String,
    name: String,
}

#[pymethods]
impl BoxerPrincipal {
    fn get_id(&self) -> &str {
        self.user_id.as_str()
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn serialize_user(&self) -> HashMap<String, String> {
        HashMap::from([
            ("id".to_string(), self.user_id.clone()),
            ("name".to_string(), self.name.clone()),
        ])
    }

    #[staticmethod]
    fn deserialize_user(token: &Bound<'_, PyDict>) -> PyResult<Self> {
        let user_id = token
            .get_item("id")?
            .ok_or_else(|| PyKeyError::new_err("id"))?
            .extract()?;
        let name = token
            .get_item("name")?
            .ok_or_else(|| PyKeyError::new_err("name"))?
            .extract()?;

        Ok(Self { user_id, name })
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
            user_id: "u1".to_string(),
            name: "User One".to_string(),
        }
        .serialize_user();

        assert_eq!(user.get("id").map(String::as_str), Some("u1"));
        assert_eq!(user.get("name").map(String::as_str), Some("User One"));
    }
}
