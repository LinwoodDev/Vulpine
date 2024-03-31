use shared::models::app::VulpineApp;

use super::ActionNamespace;

pub struct ResourceNamespace;

impl ActionNamespace for ResourceNamespace {
    fn namespace(&self) -> String {
        "resource".to_string()
    }
    fn get_actions(&self, app: &VulpineApp) -> Vec<String> {
        app.resources.keys().cloned().collect()
    }
}
