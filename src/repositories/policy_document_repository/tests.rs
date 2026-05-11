use super::*;
use boxer_core::services::backends::kubernetes::kubernetes_repository::try_into_object_ref::TryIntoObjectRef;
use boxer_core::services::backends::kubernetes::kubernetes_resource_manager::status::Status::{
    Deleted, NotFound, NotOwned,
};
use boxer_core::services::backends::kubernetes::kubernetes_resource_manager::status::not_found_details::NotFoundDetails;
use boxer_core::services::backends::kubernetes::kubernetes_resource_manager::status::owner_conflict_details::OwnerConflictDetails;
use boxer_core::testing::api_extensions::{WaitForDelete, WaitForResource};
use boxer_core::testing::spin_lock_kubernetes_resource_manager_context::GenericKubernetesResourceManagerTestContext;
use assert_matches::assert_matches;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use kube::Api;
use kube::api::PostParams;
use kube::runtime::reflector::ObjectRef;
use maplit::btreemap;
use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Duration;
use test_context::{AsyncTestContext, test_context};

use crate::models::policy_document::{PolicyDocument, PolicyDocumentSpec};

const DEFAULT_TEST_TIMEOUT: Duration = Duration::from_secs(10);

struct PolicyDocumentRepositoryTest {
    repository: Arc<PolicyDocumentRepository>,
    api: Api<PolicyDocument>,
    namespace: String,
    label: String,
}

impl AsyncTestContext for PolicyDocumentRepositoryTest {
    async fn setup() -> Self {
        let parent = GenericKubernetesResourceManagerTestContext::<PolicyDocument>::setup().await;
        let label = parent.config.owner_mark.get_owner_name().clone();
        let repository = Arc::new(
            KubernetesRepository::start(parent.manager, parent.config.operation_timeout)
                .await
                .expect("Failed to start PolicyDocumentRepository"),
        );
        Self {
            repository,
            api: parent.api_context.api,
            namespace: parent.config.namespace.clone(),
            label,
        }
    }
}

fn test_policy_data() -> PolicyDocumentData {
    PolicyDocumentData {
        policies: r#"permit(principal, action, resource);"#.to_string(),
        schema: r#"{"": {"entityTypes": {}, "actions": {}}}"#.to_string(),
    }
}

fn updated_policy_data() -> PolicyDocumentData {
    PolicyDocumentData {
        policies: r#"forbid(principal, action, resource);"#.to_string(),
        schema: r#"{"": {"entityTypes": {}, "actions": {}}}"#.to_string(),
    }
}

#[test_context(PolicyDocumentRepositoryTest)]
#[tokio::test]
async fn test_create_policy_document(ctx: &mut PolicyDocumentRepositoryTest) {
    // Arrange
    let name = "test-policy-document";
    let before = ctx.repository.get(name.to_string()).await;

    // Act
    ctx.repository
        .upsert(name.to_string(), test_policy_data())
        .await
        .expect("Failed to upsert policy document");
    ctx.api
        .wait_for_creation(
            name.to_string(),
            ctx.namespace.clone(),
            DEFAULT_TEST_TIMEOUT,
        )
        .await;

    let after = ctx.repository.get(name.to_string()).await;

    // Assert
    assert!(before.is_err());
    assert!(after.is_ok());
}

#[test_context(PolicyDocumentRepositoryTest)]
#[tokio::test]
async fn test_update_policy_document(ctx: &mut PolicyDocumentRepositoryTest) {
    // Arrange
    let name = "test-policy-document";

    ctx.repository
        .upsert(name.to_string(), test_policy_data())
        .await
        .expect("Failed to upsert policy document");
    ctx.api
        .wait_for_creation(
            name.to_string(),
            ctx.namespace.clone(),
            DEFAULT_TEST_TIMEOUT,
        )
        .await;
    let before = ctx.repository.get(name.to_string()).await.unwrap();

    // Act
    ctx.repository
        .upsert(name.to_string(), updated_policy_data())
        .await
        .expect("Failed to update policy document");
    ctx.api
        .wait_for_creation(
            name.to_string(),
            ctx.namespace.clone(),
            DEFAULT_TEST_TIMEOUT,
        )
        .await;

    let after = ctx.repository.get(name.to_string()).await.unwrap();

    // Assert
    assert_ne!(before.policies, after.policies);
}

#[test_context(PolicyDocumentRepositoryTest)]
#[tokio::test]
async fn test_delete_policy_document(ctx: &mut PolicyDocumentRepositoryTest) {
    // Arrange
    let name = "test-delete-policy-document";

    ctx.repository
        .upsert(name.to_string(), test_policy_data())
        .await
        .expect("Failed to upsert policy document");
    ctx.api
        .wait_for_creation(
            name.to_string(),
            ctx.namespace.clone(),
            DEFAULT_TEST_TIMEOUT,
        )
        .await;

    // Act
    ctx.repository
        .delete(name.to_string())
        .await
        .expect("Failed to delete policy document");
    ctx.api
        .wait_for_deletion::<PolicyDocument>(
            name.to_string(),
            ctx.namespace.clone(),
            DEFAULT_TEST_TIMEOUT,
        )
        .await;

    let after = ctx.repository.get(name.to_string()).await;

    // Assert
    assert_matches!(
        after.unwrap_err(),
        Deleted(NotFoundDetails {
            name: _,
            namespace: _,
            resource_type: rt,
        }) if rt == "PolicyDocument"
    );
}

#[test_context(PolicyDocumentRepositoryTest)]
#[tokio::test]
async fn test_policy_document_name_sanitization(ctx: &mut PolicyDocumentRepositoryTest) {
    // Arrange
    let name = "!@test-name-policy--#".to_string();
    let or: ObjectRef<PolicyDocument> = name
        .clone()
        .try_into_object_ref(ctx.namespace.clone())
        .unwrap();

    // Act
    ctx.repository
        .upsert(name.clone(), test_policy_data())
        .await
        .expect("Failed to upsert policy document");
    ctx.api
        .wait_for_creation(or.name.clone(), ctx.namespace.clone(), DEFAULT_TEST_TIMEOUT)
        .await;

    let after = ctx.repository.get(name.clone()).await;

    // Assert
    assert!(after.is_ok());
}

#[test_context(PolicyDocumentRepositoryTest)]
#[tokio::test]
async fn test_no_owner_conflict(ctx: &mut PolicyDocumentRepositoryTest) {
    // Arrange
    let name = "test-not-owned-policy";
    let resource = PolicyDocument {
        metadata: ObjectMeta {
            name: Some(name.to_string()),
            namespace: Some(ctx.namespace.clone()),
            ..Default::default()
        },
        spec: PolicyDocumentSpec::default(),
    };

    // Act
    let pp = PostParams {
        field_manager: Some("test-manager".to_string()),
        ..Default::default()
    };
    ctx.api.create(&pp, &resource).await.unwrap();
    ctx.api
        .wait_for_creation(
            name.to_string(),
            ctx.namespace.clone(),
            DEFAULT_TEST_TIMEOUT,
        )
        .await;

    let result = ctx
        .repository
        .upsert(name.to_string(), test_policy_data())
        .await;

    // Assert
    assert_matches!(
        result,
        Err(NotOwned(OwnerConflictDetails {
            name: _,
            namespace: _,
            current_owner: None,
            resource_type: _,
        }))
    );
}

#[test_context(PolicyDocumentRepositoryTest)]
#[tokio::test]
async fn test_other_owner_conflict(ctx: &mut PolicyDocumentRepositoryTest) {
    // Arrange
    let name = "test-other-owner-policy";
    let owner = "other-owner".to_string();
    let resource = PolicyDocument {
        metadata: ObjectMeta {
            labels: Some(btreemap! { ctx.label.clone() => owner.clone() }),
            name: Some(name.to_string()),
            namespace: Some(ctx.namespace.clone()),
            ..Default::default()
        },
        spec: PolicyDocumentSpec::default(),
    };

    // Act
    let pp = PostParams {
        field_manager: Some(owner.clone()),
        ..Default::default()
    };
    ctx.api.create(&pp, &resource).await.unwrap();
    ctx.api
        .wait_for_creation(
            name.to_string(),
            ctx.namespace.clone(),
            DEFAULT_TEST_TIMEOUT,
        )
        .await;

    let result = ctx
        .repository
        .upsert(name.to_string(), test_policy_data())
        .await;

    // Assert
    assert_matches!(
        result,
        Err(NotOwned(OwnerConflictDetails {
            name: _,
            namespace: _,
            current_owner: Some(_),
            resource_type: _,
        }))
    );
}

#[test_context(PolicyDocumentRepositoryTest)]
#[tokio::test]
async fn test_delete_not_owned_resource(ctx: &mut PolicyDocumentRepositoryTest) {
    // Arrange
    let name = "test-delete-not-owned-policy";
    let owner = "other-owner".to_string();
    let resource = PolicyDocument {
        metadata: ObjectMeta {
            labels: Some(BTreeMap::from([(ctx.label.clone(), owner.clone())])),
            name: Some(name.to_string()),
            namespace: Some(ctx.namespace.clone()),
            ..Default::default()
        },
        spec: PolicyDocumentSpec::default(),
    };

    let pp = PostParams {
        field_manager: Some("test-manager".to_string()),
        ..Default::default()
    };
    ctx.api.create(&pp, &resource).await.unwrap();
    ctx.api
        .wait_for_creation(
            name.to_string(),
            ctx.namespace.clone(),
            DEFAULT_TEST_TIMEOUT,
        )
        .await;

    // Act
    let after = ctx.repository.delete(name.to_string()).await;

    // Assert
    assert_eq!(
        after.as_ref().unwrap_err().to_string(),
        format!(
            "Owner conflict: Resource of kind 'PolicyDocument' with name: '{}',  namespace '{}' is not owned by us, current owner: other-owner",
            name, ctx.namespace
        )
    );
    assert_matches!(
        after,
        Err(NotOwned(OwnerConflictDetails {
            name: _,
            namespace: _,
            current_owner: Some(_),
            resource_type: _,
        }))
    );
}

#[test_context(PolicyDocumentRepositoryTest)]
#[tokio::test]
async fn test_not_existing_policy_document(ctx: &mut PolicyDocumentRepositoryTest) {
    // Arrange
    let name = "never-created-policy";

    // Act
    let after = ctx.repository.get(name.to_string()).await;

    // Assert
    assert_eq!(
        after.as_ref().unwrap_err().to_string(),
        format!(
            "Resource not found: Resource of kind 'PolicyDocument' with name: '{}',  namespace '{}' not found",
            name, ctx.namespace
        )
    );
    assert_matches!(
        after.unwrap_err(),
        NotFound(NotFoundDetails {
            name: _,
            namespace: _,
            resource_type: rt,
        }) if rt == "PolicyDocument"
    );
}

#[test_context(PolicyDocumentRepositoryTest)]
#[tokio::test]
async fn test_exists(ctx: &mut PolicyDocumentRepositoryTest) {
    // Arrange
    let name = "test-exists-policy".to_string();

    ctx.repository
        .upsert(name.clone(), test_policy_data())
        .await
        .expect("Failed to upsert policy document");
    ctx.api
        .wait_for_creation(name.clone(), ctx.namespace.clone(), DEFAULT_TEST_TIMEOUT)
        .await;

    // Act
    let after = ctx.repository.exists(name.clone()).await;

    // Assert
    assert_eq!(after.unwrap(), true);
}

#[test_context(PolicyDocumentRepositoryTest)]
#[tokio::test]
async fn test_not_exists(ctx: &mut PolicyDocumentRepositoryTest) {
    // Arrange
    let name = "never-created-policy";

    // Act
    let after = ctx.repository.exists(name.to_string()).await;

    // Assert
    assert_eq!(after.unwrap(), false);
}
