use cedar_policy::{EntityId, EntityTypeName, EntityUid};
use std::str::FromStr;

const NAMESPACE: &str = "Airflow";

/// Airflow Cedar entity kinds, covering principal types, action, and resource types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AirflowEntity {
    // Cedar structural type
    Action,
    // Principal types
    Group,
    User,
    // Resource types
    Asset,
    AssetAlias,
    Backfill,
    Configuration,
    Connection,
    Custom,
    Dag,
    Menu,
    Pool,
    Team,
    Variable,
    View,
}

impl AirflowEntity {
    pub fn as_str(self) -> &'static str {
        match self {
            AirflowEntity::Action => "Action",
            AirflowEntity::Group => "Group",
            AirflowEntity::User => "User",
            AirflowEntity::Asset => "Asset",
            AirflowEntity::AssetAlias => "AssetAlias",
            AirflowEntity::Backfill => "Backfill",
            AirflowEntity::Configuration => "Configuration",
            AirflowEntity::Connection => "Connection",
            AirflowEntity::Custom => "Custom",
            AirflowEntity::Dag => "Dag",
            AirflowEntity::Menu => "Menu",
            AirflowEntity::Pool => "Pool",
            AirflowEntity::Team => "Team",
            AirflowEntity::Variable => "Variable",
            AirflowEntity::View => "View",
        }
    }

    /// Returns the Cedar [`EntityTypeName`] for this entity kind.
    ///
    /// Example: `Airflow::Variable`, `Airflow::Action`, `Airflow::User`.
    pub fn entity_type(self) -> EntityTypeName {
        EntityTypeName::from_str(&format!("{}::{}", NAMESPACE, self.as_str()))
            .expect("statically constructed entity type name is always valid")
    }
}

/// Returns the Cedar [`EntityUid`] for an action.
///
/// A `GET` without an `entity_id` is promoted to `LIST`.
///
/// Example: `Airflow::Action::"LIST"`, `Airflow::Action::"GET"`.
pub fn action_entity_uid(method: &str, entity_id: Option<&str>) -> EntityUid {
    let effective_method = if method == "GET" && entity_id.is_none() {
        "LIST"
    } else {
        method
    };

    let action_type = AirflowEntity::Action.entity_type();
    let action_id = EntityId::new(effective_method);
    EntityUid::from_type_name_and_id(action_type, action_id)
}

#[cfg(test)]
mod tests;
