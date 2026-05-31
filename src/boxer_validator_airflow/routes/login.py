from __future__ import annotations

from fastapi import APIRouter, Request, status
from fastapi.responses import JSONResponse
from pydantic import BaseModel

from airflow.api_fastapi.app import get_cookie_path
from airflow.api_fastapi.auth.managers.base_auth_manager import COOKIE_NAME_JWT_TOKEN
from airflow.configuration import conf
from boxer_validator_airflow.boxer import Boxer


class LoginBody(BaseModel):
    external_token: str


login_router = APIRouter(tags=["BoxerAuthManagerLogin"])


@login_router.post(
    "/boxer/login/token",
    status_code=status.HTTP_201_CREATED,
)
async def create_token(body: LoginBody, request: Request) -> JSONResponse:
    """Set the Airflow cookie for an external token."""
    token = await Boxer().create_token(body.external_token)
    secure = request.base_url.scheme == "https" or bool(
        conf.get("api", "ssl_cert", fallback="")
    )

    response = JSONResponse(
        content={"access_token": token},
        status_code=status.HTTP_201_CREATED,
    )
    response.set_cookie(
        COOKIE_NAME_JWT_TOKEN,
        token,
        path=get_cookie_path(),
        secure=secure,
        httponly=True,
    )
    return response
