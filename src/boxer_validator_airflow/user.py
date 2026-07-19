from __future__ import annotations

from typing import Any

from airflow.api_fastapi.auth.managers.models.base_user import BaseUser

from boxer_validator_airflow import _core


class BoxerUser(BaseUser):
    @staticmethod
    def deserialize_user(token: dict[str, Any]) -> BoxerUser:
        normalized_token = {str(key): str(value) for key, value in token.items()}
        user = object.__new__(BoxerUser)
        user._inner = _core.BoxerPrincipal.deserialize_user(normalized_token)
        return user

    def serialize_user(self) -> dict[str, str]:
        return self._inner.serialize_user()

    def get_id(self) -> str:
        return self._inner.get_id()

    def get_name(self) -> str:
        return self._inner.get_name()
