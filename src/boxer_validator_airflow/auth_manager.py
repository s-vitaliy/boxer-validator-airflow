from __future__ import annotations

from typing import Any

from airflow.api_fastapi.auth.managers.base_auth_manager import BaseAuthManager

from boxer_validator_airflow.boxer import Boxer
from boxer_validator_airflow.user import BoxerUser


class BoxerAuthManager(BaseAuthManager[BoxerUser]):
    def __init__(self, boxer: Boxer | None = None) -> None:
        super().__init__()
        self.boxer = boxer or Boxer()

    def serialize_user(self, user: BoxerUser) -> dict[str, Any]:
        return self.boxer.serialize_user(user)

    def deserialize_user(self, token: dict[str, Any]) -> BoxerUser:
        return BoxerUser.deserialize_user(token)

    def get_url_login(self, **kwargs: Any) -> str:
        return self.boxer.get_url_login(**kwargs)

    def filter_authorized_menu_items(
        self, menu_items: list[Any], *, user: BoxerUser
    ) -> list[Any]:
        return self.boxer.filter_authorized_menu_items(
            menu_items, user_id=user.get_id()
        )

    def is_authorized_asset(
        self, *, method: str, user: BoxerUser, details: Any | None = None
    ) -> bool:
        return self.boxer.is_authorized_asset(
            method=method, user_id=user.get_id(), details=details
        )

    def is_authorized_asset_alias(
        self, *, method: str, user: BoxerUser, details: Any | None = None
    ) -> bool:
        return self.boxer.is_authorized_asset_alias(
            method=method, user_id=user.get_id(), details=details
        )

    def is_authorized_configuration(
        self, *, method: str, user: BoxerUser, details: Any | None = None
    ) -> bool:
        return self.boxer.is_authorized_configuration(
            method=method, user_id=user.get_id(), details=details
        )

    def is_authorized_connection(
        self, *, method: str, user: BoxerUser, details: Any | None = None
    ) -> bool:
        return self.boxer.is_authorized_connection(
            method=method, user_id=user.get_id(), details=details
        )

    def is_authorized_custom_view(
        self, *, method: str, resource_name: str, user: BoxerUser
    ) -> bool:
        return self.boxer.is_authorized_custom_view(
            method=method, resource_name=resource_name, user_id=user.get_id()
        )

    def is_authorized_dag(
        self,
        *,
        method: str,
        user: BoxerUser,
        access_entity: Any | None = None,
        details: Any | None = None,
    ) -> bool:
        return self.boxer.is_authorized_dag(
            method=method,
            user_id=user.get_id(),
            access_entity=access_entity,
            details=details,
        )

    def is_authorized_pool(
        self, *, method: str, user: BoxerUser, details: Any | None = None
    ) -> bool:
        return self.boxer.is_authorized_pool(
            method=method, user_id=user.get_id(), details=details
        )

    def is_authorized_variable(
        self, *, method: str, user: BoxerUser, details: Any | None = None
    ) -> bool:
        return self.boxer.is_authorized_variable(
            method=method, user_id=user.get_id(), details=details
        )

    def is_authorized_view(self, *, access_view: Any, user: BoxerUser) -> bool:
        return self.boxer.is_authorized_view(
            access_view=access_view, user_id=user.get_id()
        )
