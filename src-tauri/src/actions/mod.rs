pub mod resource;
pub mod string;

use shared::models::app::VulpineApp;
pub use string::*;
pub use resource::*;

pub trait ActionNamespace {
    fn namespace(&self) -> String;
    fn get_actions(&self, app: &VulpineApp) -> Vec<String>;
}

const NAMESPACES: [&dyn ActionNamespace; 2] = [&StringNamespace, &ResourceNamespace];

pub fn get_namespaces() -> Vec<String> {
    NAMESPACES.iter().map(|ns| ns.namespace()).collect()
}

pub fn get_namespace(namespace: &str) -> Option<&dyn ActionNamespace> {
    NAMESPACES.iter().find(|ns| ns.namespace() == namespace).copied()
}
