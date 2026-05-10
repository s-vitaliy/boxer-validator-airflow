from __future__ import annotations

from dataclasses import dataclass

from airflow.api_fastapi.auth.managers.models.base_user import BaseUser


@dataclass(frozen=True)
class BoxerUser(BaseUser):
    user_id: str
    name: str

    def get_id(self) -> str:
        return self.user_id

    def get_name(self) -> str:
        return self.name
