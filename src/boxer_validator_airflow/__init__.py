from typing import Any

from boxer_validator_airflow.boxer import Boxer


def __getattr__(name: str) -> Any:
    if name == "BoxerAuthManager":
        from boxer_validator_airflow.auth_manager import BoxerAuthManager

        return BoxerAuthManager
    if name == "BoxerUser":
        from boxer_validator_airflow.user import BoxerUser

        return BoxerUser
    raise AttributeError(f"module {__name__!r} has no attribute {name!r}")


__all__ = ["Boxer", "BoxerAuthManager", "BoxerUser"]
