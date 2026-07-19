from fastapi import FastAPI
from fastapi.testclient import TestClient

from boxer_validator_airflow.routes.login import login_router


def test_create_token_sets_airflow_cookie(monkeypatch) -> None:
    async def create_token(_self, _external_token: str) -> dict[str, str]:
        return {
            "boxer.sneaksanddata.com/principal": "principal-1",
            "boxer.sneaksanddata.com/external-identity": "user-1",
            "boxer.sneaksanddata.com/identity-provider": "keycloak",
        }

    class AuthManager:
        def generate_jwt(self, *, user) -> str:
            assert user.get_id() == "keycloak/user-1"
            return "airflow-token"

    monkeypatch.setattr("boxer_validator_airflow.routes.login.Boxer.create_token", create_token)
    monkeypatch.setattr(
        "boxer_validator_airflow.routes.login.get_auth_manager",
        lambda: AuthManager(),
    )

    app = FastAPI()
    app.include_router(login_router)

    response = TestClient(app).post(
        "/boxer/login/token",
        json={"external_token": "external-token"},
    )

    assert response.status_code == 201
    assert response.json() == {"access_token": "airflow-token"}
    assert response.cookies["_token"] == "airflow-token"
    assert "HttpOnly" in response.headers["set-cookie"]
