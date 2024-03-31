use shared::models::app::VulpineApp;

use super::ActionNamespace;

pub struct StringNamespace;

impl ActionNamespace for StringNamespace {
    fn namespace(&self) -> String {
        "string".to_string()
    }
    fn get_actions(&self, _app: &VulpineApp) -> Vec<String> {
        vec!["to_uppercase".to_string(), "to_lowercase".to_string(), "trim".to_string(), "replace".to_string()]
    }
}
