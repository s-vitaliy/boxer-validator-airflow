from __future__ import annotations

from typing import Any

from fastapi import FastAPI
from starlette.middleware import _MiddlewareFactory

from airflow.api_fastapi.auth.managers.base_auth_manager import BaseAuthManager

from boxer_validator_airflow.boxer import Boxer
from boxer_validator_airflow.user import BoxerUser


API_ROOT_PATH = "/"

# Define the full path on which the potential auth manager fastapi is mounted
AUTH_MANAGER_FASTAPI_APP_PREFIX = f"{API_ROOT_PATH}auth"


class BoxerAuthManager(BaseAuthManager[BoxerUser]):
    def __init__(self, boxer: Boxer | None = None) -> None:
        super().__init__()
        self.boxer = boxer or Boxer()

    def serialize_user(self, user: BoxerUser) -> dict[str, Any]:
        return self.boxer.serialize_user(user)

    def deserialize_user(self, token: dict[str, Any]) -> BoxerUser:
        return BoxerUser.deserialize_user(token)

    def get_url_login(self, **kwargs: Any) -> str:
        return f"{AUTH_MANAGER_FASTAPI_APP_PREFIX}/boxer/login"

    def filter_authorized_menu_items(
        self, menu_items: list[Any], *, user: BoxerUser
    ) -> list[Any]:
        return self.boxer.filter_authorized_menu_items(
            menu_items, user_id=user.get_id()
        )

    def is_authorized_asset(
        self, *, method: str, user: BoxerUser, details: Any | None = None
    ) -> bool:
        return self.boxer.is_authorized(
            method=method, resource_type="Asset", user_id=user.get_id()
        )

    def is_authorized_asset_alias(
        self, *, method: str, user: BoxerUser, details: Any | None = None
    ) -> bool:
        return self.boxer.is_authorized(
            method=method, resource_type="AssetAlias", user_id=user.get_id()
        )

    def is_authorized_configuration(
        self, *, method: str, user: BoxerUser, details: Any | None = None
    ) -> bool:
        # return self.boxer.is_authorized(
        #     method=method, resource_type="Configuration", user_id=user.get_id()
        # )
        # TODO
        return True

    def is_authorized_connection(
        self, *, method: str, user: BoxerUser, details: Any | None = None
    ) -> bool:
        return self.boxer.is_authorized(
            method=method, resource_type="Connection", user_id=user.get_id()
        )

    def is_authorized_custom_view(
        self, *, method: str, resource_name: str, user: BoxerUser
    ) -> bool:
        return self.boxer.is_authorized(
            method=method,
            resource_type="Custom",
            user_id=user.get_id(),
            entity_id=resource_name,
        )

    def is_authorized_dag(
        self,
        *,
        method: str,
        user: BoxerUser,
        access_entity: Any | None = None,
        details: Any | None = None,
    ) -> bool:
        return self.boxer.is_authorized(
            method=method, resource_type="Dag", user_id=user.get_id()
        )

    def is_authorized_pool(
        self, *, method: str, user: BoxerUser, details: Any | None = None
    ) -> bool:
        return self.boxer.is_authorized(
            method=method, resource_type="Pool", user_id=user.get_id()
        )

    def is_authorized_variable(
        self, *, method: str, user: BoxerUser, details: Any | None = None
    ) -> bool:
        return self.boxer.is_authorized(
            method=method, resource_type="Variable", user_id=user.get_id()
        )

    def is_authorized_view(self, *, access_view: Any, user: BoxerUser) -> bool:
        return self.boxer.is_authorized(
            method="GET", resource_type="View", user_id=user.get_id()
        )

    def get_fastapi_middlewares(
        self,
    ) -> list[tuple[_MiddlewareFactory[Any], dict[str, Any]]]:
        """Register the all-admins middleware when ``[core] simple_auth_manager_all_admins`` is set."""
        # if not conf.getboolean("core", "simple_auth_manager_all_admins"):
        return []
        # from airflow.api_fastapi.auth.managers.simple.middleware import SimpleAllAdminMiddleware

        # return [(SimpleAllAdminMiddleware, {})]



    def get_fastapi_app(self) -> FastAPI | None:
        """
        Specify a sub FastAPI application specific to the auth manager.

        This sub application, if specified, is mounted in the main FastAPI application.
        """
        from airflow.api_fastapi.auth.managers.simple.routes.login import login_router
        from fastapi.responses import PlainTextResponse

        app = FastAPI(title="Boxer auth manager sub application")
        app.include_router(login_router)

        @app.get("/{rest_of_path:path}", include_in_schema=False)
        def webapp(rest_of_path: str) -> PlainTextResponse:
            return PlainTextResponse("hello")

        return app
