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

    pub fn is_authorized_asset(&self, _method: &str, _user_id: &str) -> bool {
        false
    }

    pub fn is_authorized_asset_alias(&self, _method: &str, _user_id: &str) -> bool {
        false
    }

    pub fn is_authorized_configuration(&self, _method: &str, _user_id: &str) -> bool {
        false
    }

    pub fn is_authorized_connection(&self, _method: &str, _user_id: &str) -> bool {
        false
    }

    pub fn is_authorized_custom_view(
        &self,
        _method: &str,
        _resource_name: &str,
        _user_id: &str,
    ) -> bool {
        false
    }

    pub fn is_authorized_dag(&self, _method: &str, _user_id: &str) -> bool {
        false
    }

    pub fn is_authorized_pool(&self, _method: &str, _user_id: &str) -> bool {
        false
    }

    pub fn is_authorized_variable(&self, _method: &str, _user_id: &str) -> bool {
        false
    }

    pub fn is_authorized_view(&self, _access_view: &str, _user_id: &str) -> bool {
        false
    }
}
