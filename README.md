# boxer-validator-airflow

Rust library with a Python wrapper built via `uv` and `maturin`.

## Quick start

```bash
uv sync
uv run python -c "from boxer_validator_airflow import BoxerAuthManager; print(BoxerAuthManager.__name__)"
```

## Configuration

`BOXER_ISSUER_BASE_URL` overrides the Boxer issuer base URL. It defaults to
`http://localhost:5555/issuer/`.

`IDENTITY_PROVIDER` selects the Boxer issuer identity provider. It defaults to
`keycloak`.
