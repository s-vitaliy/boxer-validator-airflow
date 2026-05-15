use cedar_policy::{Authorizer, Context, Decision, Entities, EntityUid, PolicySet, Request};

pub struct Boxer {
    policies: PolicySet,
    authorizer: Authorizer,
}

impl Boxer {
    pub fn new(policies: PolicySet) -> Self {
        Self {
            policies,
            authorizer: Authorizer::new(),
        }
    }

    pub fn get_url_login(&self) -> String {
        String::new()
    }

    pub fn filter_authorized_menu_items<T>(&self, _menu_items: Vec<T>, _user_id: &str) -> Vec<T> {
        Vec::new()
    }

    pub fn is_authorized(&self, principal: EntityUid, action: EntityUid, resource: EntityUid) -> bool {
        let Ok(request) = Request::new(principal, action, resource, Context::empty(), None) else {
            return false;
        };
        self.authorizer
            .is_authorized(&request, &self.policies, &Entities::empty())
            .decision() == Decision::Allow
    }
}
