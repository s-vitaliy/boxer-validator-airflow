use std::sync::Arc;

use cedar_policy::PolicySet;

use super::Boxer;
use crate::services::static_policy_repository::StaticPolicyRepository;

#[tokio::test]
async fn creates_mvp_token() {
    let repository = Arc::new(StaticPolicyRepository::new(PolicySet::default()));
    let boxer = Boxer::new(repository);

    assert_eq!(boxer.create_token("external-token").await, "token");
}
