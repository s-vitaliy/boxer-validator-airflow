use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use cedar_policy::PolicySet;

use super::Boxer;
use crate::services::static_policy_repository::StaticPolicyRepository;
use crate::services::token_issuer::{TokenIssuer, TokenIssuerError};

#[derive(Default)]
struct FakeTokenIssuer {
    calls: Mutex<Vec<(String, String, String)>>,
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
        Ok("issuer-token".to_string())
    }
}

#[tokio::test]
async fn creates_token_through_issuer() {
    let repository = Arc::new(StaticPolicyRepository::new(PolicySet::default()));
    let token_issuer = Arc::new(FakeTokenIssuer::default());
    let boxer = Boxer::with_token_issuer(repository, token_issuer.clone());

    assert_eq!(
        boxer
            .create_token("external-token")
            .await
            .expect("issuer token"),
        "issuer-token"
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
