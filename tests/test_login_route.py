from fastapi import FastAPI
from fastapi.testclient import TestClient

from boxer_validator_airflow.routes.login import login_router


def test_create_token_sets_airflow_cookie(monkeypatch) -> None:
    async def create_token(_self, _external_token: str) -> str:
        return "token"

    monkeypatch.setattr("boxer_validator_airflow.routes.login.Boxer.create_token", create_token)

    app = FastAPI()
    app.include_router(login_router)

    response = TestClient(app).post(
        "/boxer/login/token",
        json={"external_token": "external-token"},
    )

    assert response.status_code == 201
    assert response.json() == {"access_token": "token"}
    assert response.cookies["_token"] == "token"
    assert "HttpOnly" in response.headers["set-cookie"]
