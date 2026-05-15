use async_trait::async_trait;
use boxer_core::services::base::upsert_repository::ReadOnlyRepository;
use cedar_policy::PolicySet;

/// A simple in-memory repository that holds a fixed [`PolicySet`].
pub struct StaticPolicyRepository(PolicySet);

impl StaticPolicyRepository {
    pub fn new(policy_set: PolicySet) -> Self {
        Self(policy_set)
    }
}

#[async_trait]
impl ReadOnlyRepository<(), PolicySet> for StaticPolicyRepository {
    type ReadError = Box<dyn std::error::Error + Send + Sync>;

    async fn get(&self, _key: ()) -> Result<PolicySet, Self::ReadError> {
        Ok(self.0.clone())
    }
}
