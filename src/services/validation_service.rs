use boxer_core::services::base::upsert_repository::ReadOnlyRepository;
use cedar_policy::{Authorizer, Context, Decision, Entities, EntityUid, PolicySet, Request};
use std::sync::Arc;

#[cfg(test)]
mod tests;

type PolicyRepository =
    dyn ReadOnlyRepository<(), PolicySet, ReadError = Box<dyn std::error::Error + Send + Sync>>;

pub struct Boxer {
    repository: Arc<PolicyRepository>,
    authorizer: Authorizer,
}

impl Boxer {
    pub fn new(repository: Arc<PolicyRepository>) -> Self {
        Self {
            repository,
            authorizer: Authorizer::new(),
        }
    }

    pub fn get_url_login(&self) -> String {
        String::new()
    }

    pub async fn create_token(&self, _external_token: &str) -> String {
        "token".to_string()
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
