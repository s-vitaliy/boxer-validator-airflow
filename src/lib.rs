use pyo3::{prelude::*, wrap_pyfunction};

pub fn hello() -> String {
    "Hello from boxer-validator-airflow!".to_string()
}

#[pyfunction]
fn hello_from_rust() -> String {
    hello()
}

/// A Python module implemented in Rust. The name of this module must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello_from_rust, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::hello;

    #[test]
    fn returns_expected_greeting() {
        assert_eq!(hello(), "Hello from boxer-validator-airflow!");
    }
}
