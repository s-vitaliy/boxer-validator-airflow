use boxer_core::services::base::upsert_repository::ReadOnlyRepository;
use cedar_policy::{Authorizer, Context, Decision, Entities, EntityUid, PolicySet, Request};
use std::env;
use std::sync::Arc;

use crate::services::token_issuer::{ReqwestTokenIssuer, TokenIssuer, TokenIssuerError};

#[cfg(test)]
mod tests;

type PolicyRepository =
    dyn ReadOnlyRepository<(), PolicySet, ReadError = Box<dyn std::error::Error + Send + Sync>>;

const BOXER_ISSUER_BASE_URL_ENV_VAR: &str = "BOXER_ISSUER_BASE_URL";
const DEFAULT_BOXER_ISSUER_BASE_URL: &str = "http://localhost:5555/issuer/";
const IDENTITY_PROVIDER_ENV_VAR: &str = "IDENTITY_PROVIDER";
const DEFAULT_IDENTITY_PROVIDER: &str = "keycloak";

pub struct Boxer {
    repository: Arc<PolicyRepository>,
    authorizer: Authorizer,
    token_issuer: Arc<dyn TokenIssuer>,
}

impl Boxer {
    pub fn new(repository: Arc<PolicyRepository>) -> Self {
        Self::with_token_issuer(repository, Arc::new(ReqwestTokenIssuer::new()))
    }

    pub fn with_token_issuer(
        repository: Arc<PolicyRepository>,
        token_issuer: Arc<dyn TokenIssuer>,
    ) -> Self {
        Self {
            repository,
            authorizer: Authorizer::new(),
            token_issuer,
        }
    }

    pub fn get_url_login(&self) -> String {
        String::new()
    }

    pub async fn create_token(&self, external_token: &str) -> Result<String, TokenIssuerError> {
        let base_url = env::var(BOXER_ISSUER_BASE_URL_ENV_VAR)
            .unwrap_or_else(|_| DEFAULT_BOXER_ISSUER_BASE_URL.to_string());
        let identity_provider = env::var(IDENTITY_PROVIDER_ENV_VAR)
            .unwrap_or_else(|_| DEFAULT_IDENTITY_PROVIDER.to_string());
        self.token_issuer
            .create_token(external_token, &base_url, &identity_provider)
            .await
    }

    pub fn filter_authorized_menu_items<T>(&self, _menu_items: Vec<T>, _user_id: &str) -> Vec<T> {
        Vec::new()
    }

    pub async fn is_authorized(
        &self,
        principal: EntityUid,
        action: EntityUid,
        resource: EntityUid,
    ) -> bool {
        let Ok(policies) = self.repository.get(()).await else {
            return false;
        };
        let Ok(request) = Request::new(principal, action, resource, Context::empty(), None) else {
            return false;
        };
        self.authorizer
            .is_authorized(&request, &policies, &Entities::empty())
            .decision()
            == Decision::Allow
    }
}
