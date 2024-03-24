use std::collections::HashMap;
use indexmap::IndexMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct VulpineApp {
    pub name: String,
    pub description: String,
    pub version: String,
    pub actions: IndexMap<String, VulpineAction>,
    pub resources: IndexMap<String, VulpineResource>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct VulpineResource {
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum VulpineStep {
    Command {
        executable: String,
        args: Vec<HashMap<String, ValueMapping>>,
        env: HashMap<String, ValueMapping>,
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct VulpineAction {
    pub description: String,
    pub inputs: HashMap<String, ActionInput>,
    pub steps: Vec<VulpineStep>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ActionInput {
    Checkbox {
        label: String,
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ValueMapping {
    Constant {
        value: String,
    },
    Input {
        input: String,
        mapping: HashMap<String, String>,
    }
}

#[derive(Serialize, Deserialize)]
pub struct AppName(String);

impl AppName {
    pub fn parse(name: &str) -> Option<Self> {
        let name = safe_filename(name);
        if name.is_empty() {
            None
        } else {
            Some(Self(name))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn as_filename(&self) -> String {
        format!("{}.toml", self.0)
    }
}

const ALLOWED_SPECIAL_CHARS: &str = "_- ";

pub fn safe_filename(name: &str) -> String {
    name.trim().replace(|c: char| !c.is_ascii_alphanumeric() && !ALLOWED_SPECIAL_CHARS.contains(c), "")
}
