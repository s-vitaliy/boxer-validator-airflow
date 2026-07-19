use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use cedar_policy::PolicySet;
use josekit::Value;
use josekit::jwe::{Dir, JweHeader};
use josekit::jwt::{self, JwtPayload};
use serde_json::json;

use super::Boxer;
use crate::models::boxer_principal::{IDENTITY_PROVIDER_KEY, PRINCIPAL_KEY, USER_ID_KEY};
use crate::services::static_policy_repository::StaticPolicyRepository;
use crate::services::token_issuer::{TokenIssuer, TokenIssuerError};

#[derive(Default)]
struct FakeTokenIssuer {
    calls: Mutex<Vec<(String, String, String)>>,
    token: String,
}

#[async_trait]
impl TokenIssuer for FakeTokenIssuer {
    async fn create_token(
        &self,
        external_token: &str,
        base_url: &str,
        identity_provider: &str,
    ) -> Result<String, TokenIssuerError> {
        self.calls.lock().expect("calls lock").push((
            external_token.to_string(),
            base_url.to_string(),
            identity_provider.to_string(),
        ));
        Ok(self.token.clone())
    }
}

#[tokio::test]
async fn creates_token_through_issuer() {
    let jwe_key = b"01234567890123456789012345678901";
    let repository = Arc::new(StaticPolicyRepository::new(PolicySet::default()));
    let token_issuer = Arc::new(FakeTokenIssuer {
        token: issuer_jwe(jwe_key),
        ..Default::default()
    });
    let boxer = Boxer::with_token_issuer(repository, token_issuer.clone());

    assert_eq!(
        boxer
            .create_token_with_jwe_key("external-token", jwe_key)
            .await
            .expect("issuer token"),
        [
            (PRINCIPAL_KEY.to_string(), "principal-1".to_string()),
            (USER_ID_KEY.to_string(), "user-1".to_string()),
            (IDENTITY_PROVIDER_KEY.to_string(), "keycloak".to_string()),
        ]
        .into()
    );
    assert_eq!(
        *token_issuer.calls.lock().expect("calls lock"),
        vec![(
            "external-token".to_string(),
            "http://localhost:5555/issuer/".to_string(),
            "keycloak".to_string(),
        )]
    );
}

#[tokio::test]
async fn serializes_object_claims_to_json_strings() {
    let jwe_key = b"01234567890123456789012345678901";
    let repository = Arc::new(StaticPolicyRepository::new(PolicySet::default()));
    let token_issuer = Arc::new(FakeTokenIssuer {
        token: issuer_jwe_with_object_principal(jwe_key),
        ..Default::default()
    });
    let boxer = Boxer::with_token_issuer(repository, token_issuer);

    let token = boxer
        .create_token_with_jwe_key("external-token", jwe_key)
        .await
        .expect("issuer token");
    let principal = token.get(PRINCIPAL_KEY).expect("principal claim");
    assert_eq!(
        serde_json::from_str::<Value>(principal).expect("principal JSON"),
        json!({
            "attrs": {},
            "parents": [],
            "uid": {"id": "root", "type": "PhotoApp::User"}
        })
    );
}

#[tokio::test]
async fn serializes_all_claim_types_to_strings() {
    let jwe_key = b"01234567890123456789012345678901";
    let repository = Arc::new(StaticPolicyRepository::new(PolicySet::default()));
    let token_issuer = Arc::new(FakeTokenIssuer {
        token: issuer_jwe_with_mixed_claim_types(jwe_key),
        ..Default::default()
    });
    let boxer = Boxer::with_token_issuer(repository, token_issuer);

    let token = boxer
        .create_token_with_jwe_key("external-token", jwe_key)
        .await
        .expect("issuer token");

    assert_eq!(token.get("exp"), Some(&"1784459684.175976".to_string()));
    assert_eq!(token.get("active"), Some(&"true".to_string()));
    assert_eq!(token.get("groups"), Some(&r#"["ops","admin"]"#.to_string()));
    assert_eq!(token.get("meta"), Some(&r#"{"k":"v"}"#.to_string()));
    assert_eq!(token.get("optional"), Some(&"null".to_string()));
}

fn issuer_jwe(jwe_key: &[u8]) -> String {
    let mut payload = JwtPayload::new();
    payload
        .set_claim(
            PRINCIPAL_KEY,
            Some(Value::String("principal-1".to_string())),
        )
        .expect("principal claim");
    payload
        .set_claim(USER_ID_KEY, Some(Value::String("user-1".to_string())))
        .expect("user id claim");
    payload
        .set_claim(
            IDENTITY_PROVIDER_KEY,
            Some(Value::String("keycloak".to_string())),
        )
        .expect("identity provider claim");

    let mut header = JweHeader::new();
    header.set_content_encryption("A256GCM");
    let encrypter = Dir.encrypter_from_bytes(jwe_key).expect("JWE encrypter");

    jwt::encode_with_encrypter(&payload, &header, &encrypter).expect("issuer JWE")
}

fn issuer_jwe_with_object_principal(jwe_key: &[u8]) -> String {
    let mut payload = JwtPayload::new();
    payload
        .set_claim(
            PRINCIPAL_KEY,
            Some(json!({
                "uid": {"type": "PhotoApp::User", "id": "root"},
                "attrs": {},
                "parents": []
            })),
        )
        .expect("principal claim");
    payload
        .set_claim(USER_ID_KEY, Some(Value::String("user-1".to_string())))
        .expect("user id claim");
    payload
        .set_claim(
            IDENTITY_PROVIDER_KEY,
            Some(Value::String("keycloak".to_string())),
        )
        .expect("identity provider claim");

    let mut header = JweHeader::new();
    header.set_content_encryption("A256GCM");
    let encrypter = Dir.encrypter_from_bytes(jwe_key).expect("JWE encrypter");

    jwt::encode_with_encrypter(&payload, &header, &encrypter).expect("issuer JWE")
}

fn issuer_jwe_with_mixed_claim_types(jwe_key: &[u8]) -> String {
    let mut payload = JwtPayload::new();
    payload
        .set_claim(
            PRINCIPAL_KEY,
            Some(Value::String("principal-1".to_string())),
        )
        .expect("principal claim");
    payload
        .set_claim(USER_ID_KEY, Some(Value::String("user-1".to_string())))
        .expect("user id claim");
    payload
        .set_claim(
            IDENTITY_PROVIDER_KEY,
            Some(Value::String("keycloak".to_string())),
        )
        .expect("identity provider claim");
    payload
        .set_claim("exp", Some(json!(1784459684.175976)))
        .expect("exp claim");
    payload
        .set_claim("active", Some(json!(true)))
        .expect("active claim");
    payload
        .set_claim("groups", Some(json!(["ops", "admin"])))
        .expect("groups claim");
    payload
        .set_claim("meta", Some(json!({"k": "v"})))
        .expect("meta claim");
    payload
        .set_claim("optional", Some(Value::Null))
        .expect("optional claim");

    let mut header = JweHeader::new();
    header.set_content_encryption("A256GCM");
    let encrypter = Dir.encrypter_from_bytes(jwe_key).expect("JWE encrypter");

    jwt::encode_with_encrypter(&payload, &header, &encrypter).expect("issuer JWE")
}
