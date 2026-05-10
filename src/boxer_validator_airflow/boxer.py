from __future__ import annotations

from typing import Any

from boxer_validator_airflow import _core


class Boxer:
    def serialize_user(self, *, user_id: str, name: str) -> dict[str, str]:
        return _core.serialize_user(user_id, name)

    def deserialize_user(self, token: dict[str, Any]) -> dict[str, str]:
        user_id, name = _core.deserialize_user(token)
        return {"id": user_id, "name": name}

    def get_url_login(self, **kwargs: Any) -> str:
        return _core.get_url_login(kwargs)

    def filter_authorized_menu_items(
        self, menu_items: list[Any], *, user_id: str
    ) -> list[Any]:
        return _core.filter_authorized_menu_items(menu_items, user_id)

    def is_authorized_asset(
        self, *, method: str, user_id: str, details: Any | None = None
    ) -> bool:
        return _core.is_authorized_asset(method, user_id, details)

    def is_authorized_asset_alias(
        self, *, method: str, user_id: str, details: Any | None = None
    ) -> bool:
        return _core.is_authorized_asset_alias(method, user_id, details)

    def is_authorized_configuration(
        self, *, method: str, user_id: str, details: Any | None = None
    ) -> bool:
        return _core.is_authorized_configuration(method, user_id, details)

    def is_authorized_connection(
        self, *, method: str, user_id: str, details: Any | None = None
    ) -> bool:
        return _core.is_authorized_connection(method, user_id, details)

    def is_authorized_custom_view(
        self, *, method: str, resource_name: str, user_id: str
    ) -> bool:
        return _core.is_authorized_custom_view(method, resource_name, user_id)

    def is_authorized_dag(
        self,
        *,
        method: str,
        user_id: str,
        access_entity: Any | None = None,
        details: Any | None = None,
    ) -> bool:
        return _core.is_authorized_dag(method, user_id, access_entity, details)

    def is_authorized_pool(
        self, *, method: str, user_id: str, details: Any | None = None
    ) -> bool:
        return _core.is_authorized_pool(method, user_id, details)

    def is_authorized_variable(
        self, *, method: str, user_id: str, details: Any | None = None
    ) -> bool:
        return _core.is_authorized_variable(method, user_id, details)

    def is_authorized_view(self, *, access_view: Any, user_id: str) -> bool:
        return _core.is_authorized_view(access_view, user_id)
