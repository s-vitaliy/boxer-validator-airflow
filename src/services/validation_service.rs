use cedar_policy::EntityUid;

pub struct Boxer;

impl Boxer {
    pub fn new() -> Self {
        Self
    }

    pub fn get_url_login(&self) -> String {
        String::new()
    }

    pub fn filter_authorized_menu_items<T>(&self, _menu_items: Vec<T>, _user_id: &str) -> Vec<T> {
        Vec::new()
    }

    pub fn is_authorized(&self, _action: EntityUid, _user: EntityUid, _resource: EntityUid) -> bool {
        false
    }
}
