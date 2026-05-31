from __future__ import annotations

from typing import Any

from boxer_validator_airflow import _core


class Boxer:
    def __init__(self) -> None:
        self._inner = _core.Boxer()

    def serialize_user(self, user: Any) -> dict[str, str]:
        return user.serialize_user()

    def deserialize_user(self, token: dict[str, Any]) -> Any:
        return _core.BoxerPrincipal.deserialize_user(token)

    def get_url_login(self, **kwargs: Any) -> str:
        return self._inner.get_url_login(kwargs)

    async def create_token(self, external_token: str) -> str:
        return await self._inner.create_token(external_token)

    def filter_authorized_menu_items(
        self, menu_items: list[Any], *, user_id: str
    ) -> list[Any]:
        return self._inner.filter_authorized_menu_items(menu_items, user_id)

    def is_authorized(
        self,
        *,
        method: str,
        resource_type: str,
        user_id: str,
        entity_id: str | None = None,
    ) -> bool:
        return self._inner.is_authorized(method, resource_type, user_id, entity_id)
