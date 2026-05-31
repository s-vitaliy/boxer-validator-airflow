#[cfg(test)]
mod tests;

use boxer_core::services::backends::kubernetes::kubernetes_repository::KubernetesRepository;
use boxer_core::services::backends::kubernetes::kubernetes_repository::to_resource::ToResource;
use boxer_core::services::backends::kubernetes::kubernetes_repository::try_from_resource::TryFromResource;
use boxer_core::services::backends::kubernetes::kubernetes_resource_manager::GenericKubernetesResourceManager;
use boxer_core::services::backends::kubernetes::kubernetes_resource_manager::status::Status;
use boxer_core::services::base::upsert_repository::UpsertRepositoryWithDelete;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use std::sync::Arc;

use crate::models::policy_document::{PolicyDocument, PolicyDocumentData, PolicyDocumentSpec};

impl ToResource<PolicyDocument> for PolicyDocumentData {
    fn to_resource(&self, object_meta: &ObjectMeta) -> Result<PolicyDocument, Status> {
        Ok(PolicyDocument {
            metadata: object_meta.clone(),
            spec: PolicyDocumentSpec {
                active: true,
                policies: self.policies.clone(),
                schema: self.schema.clone(),
            },
        })
    }
}

impl TryFromResource<PolicyDocument> for PolicyDocumentData {
    type Error = Status;

    fn try_from_resource(resource: Arc<PolicyDocument>) -> Result<Self, Self::Error> {
        Ok(PolicyDocumentData {
            policies: resource.spec.policies.clone(),
            schema: resource.spec.schema.clone(),
        })
    }
}

impl UpsertRepositoryWithDelete<String, PolicyDocumentData>
    for KubernetesRepository<PolicyDocument, GenericKubernetesResourceManager<PolicyDocument>>
{
}

pub type PolicyDocumentRepository = dyn UpsertRepositoryWithDelete<
        String,
        PolicyDocumentData,
        DeleteError = Status,
        Error = Status,
        ReadError = Status,
    >;
