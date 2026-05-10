use std::collections::HashMap;

use super::{BoxerPrincipal, IDENTITY_PROVIDER_KEY, PRINCIPAL_KEY, USER_ID_KEY};

fn token() -> HashMap<String, String> {
    HashMap::from([
        (PRINCIPAL_KEY.to_string(), "principal-1".to_string()),
        (USER_ID_KEY.to_string(), "u1".to_string()),
        (IDENTITY_PROVIDER_KEY.to_string(), "idp".to_string()),
    ])
}

#[test]
fn serializes_user() {
    let user = BoxerPrincipal::deserialize_user(&token()).expect("valid token");

    assert_eq!(user.serialize_user(), token());
}

#[test]
fn formats_user_id_from_identity_provider_and_external_identity() {
    let user = BoxerPrincipal::deserialize_user(&token()).expect("valid token");

    assert_eq!(user.get_id(), "idp/u1");
    assert_eq!(user.get_name(), "idp/u1");
}

#[test]
fn requires_principal_claim() {
    let mut token = token();
    token.remove(PRINCIPAL_KEY);

    let error = BoxerPrincipal::deserialize_user(&token).expect_err("missing claim");

    assert_eq!(error.key(), PRINCIPAL_KEY);
}
