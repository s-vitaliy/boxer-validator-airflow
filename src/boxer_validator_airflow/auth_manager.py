from __future__ import annotations

from typing import Any

from airflow.api_fastapi.auth.managers.base_auth_manager import BaseAuthManager

from boxer_validator_airflow.user import BoxerUser


class BoxerAuthManager(BaseAuthManager[BoxerUser]):
    def serialize_user(self, user: BoxerUser) -> dict[str, Any]:
        return {
            "id": user.get_id(),
            "name": user.get_name(),
        }

    def deserialize_user(self, token: dict[str, Any]) -> BoxerUser:
        return BoxerUser(
            user_id=str(token["id"]),
            name=str(token["name"]),
        )

    def get_url_login(self, **kwargs: Any) -> str:
        raise NotImplementedError("Login URL resolution is implemented by the Boxer auth boundary.")

    def filter_authorized_menu_items(self, menu_items: list[Any], *, user: BoxerUser) -> list[Any]:
        return []

    def is_authorized_asset(
        self, *, method: str, user: BoxerUser, details: Any | None = None
    ) -> bool:
        return False

    def is_authorized_asset_alias(
        self, *, method: str, user: BoxerUser, details: Any | None = None
    ) -> bool:
        return False

    def is_authorized_configuration(
        self, *, method: str, user: BoxerUser, details: Any | None = None
    ) -> bool:
        return False

    def is_authorized_connection(
        self, *, method: str, user: BoxerUser, details: Any | None = None
    ) -> bool:
        return False

    def is_authorized_custom_view(
        self, *, method: str, resource_name: str, user: BoxerUser
    ) -> bool:
        return False

    def is_authorized_dag(
        self,
        *,
        method: str,
        user: BoxerUser,
        access_entity: Any | None = None,
        details: Any | None = None,
    ) -> bool:
        return False

    def is_authorized_pool(
        self, *, method: str, user: BoxerUser, details: Any | None = None
    ) -> bool:
        return False

    def is_authorized_variable(
        self, *, method: str, user: BoxerUser, details: Any | None = None
    ) -> bool:
        return False

    def is_authorized_view(self, *, access_view: Any, user: BoxerUser) -> bool:
        return False
