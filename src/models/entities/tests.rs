use super::*;
use std::str::FromStr;

#[test]
fn entity_type_returns_namespaced_type_name() {
    let ty = AirflowEntity::Variable.entity_type();
    assert_eq!(ty, EntityTypeName::from_str("Airflow::Variable").unwrap());
}

#[test]
fn action_uid_get_with_entity_id_stays_get() {
    let uid = action_entity_uid("GET", Some("my_dag"));
    assert_eq!(uid.id().unescaped(), "GET");
    assert_eq!(
        uid.type_name(),
        &EntityTypeName::from_str("Airflow::Action").unwrap()
    );
}

#[test]
fn action_uid_get_without_entity_id_becomes_list() {
    let uid = action_entity_uid("GET", None);
    assert_eq!(uid.id().unescaped(), "LIST");
}

#[test]
fn action_uid_non_get_method_is_unchanged() {
    let uid = action_entity_uid("POST", None);
    assert_eq!(uid.id().unescaped(), "POST");
}
