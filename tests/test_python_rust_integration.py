from __future__ import annotations

import pytest

from boxer_validator_airflow import BoxerAuthManager, BoxerUser


PRINCIPAL_KEY = "boxer.sneaksanddata.com/principal"
USER_ID_KEY = "boxer.sneaksanddata.com/external-identity"
IDENTITY_PROVIDER_KEY = "boxer.sneaksanddata.com/identity-provider"


@pytest.fixture
def token() -> dict[str, str]:
    return {
        PRINCIPAL_KEY: "principal-1",
        USER_ID_KEY: "user-1",
        IDENTITY_PROVIDER_KEY: "test-idp",
    }


def test_boxer_user_deserializes_token_using_rust_principal(token: dict[str, str]) -> None:
    user = BoxerUser.deserialize_user(token)

    assert user.get_id() == "test-idp/user-1"
    assert user.get_name() == "test-idp/user-1"


def test_boxer_user_serializes_token_using_rust_principal(token: dict[str, str]) -> None:
    user = BoxerUser.deserialize_user(token)

    assert user.serialize_user() == token


def test_auth_manager_serializes_user_through_python_wrapper_and_rust(
    token: dict[str, str],
) -> None:
    manager = BoxerAuthManager()
    user = BoxerUser.deserialize_user(token)

    assert manager.serialize_user(user) == token


def test_auth_manager_deserializes_user_through_python_wrapper_and_rust(
    token: dict[str, str],
) -> None:
    user = BoxerAuthManager().deserialize_user(token)

    assert isinstance(user, BoxerUser)
    assert user.get_id() == "test-idp/user-1"
    assert user.serialize_user() == token


@pytest.mark.parametrize(
    "missing_key",
    [PRINCIPAL_KEY, USER_ID_KEY, IDENTITY_PROVIDER_KEY],
)
def test_boxer_user_requires_all_principal_claims(
    token: dict[str, str], missing_key: str
) -> None:
    del token[missing_key]

    with pytest.raises(KeyError, match=missing_key):
        BoxerUser.deserialize_user(token)
