use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct VulpineApp {
    pub name: String,
    pub description: String,
    pub version: String,
    pub actions: BTreeMap<String, VulpineAction>,
    pub executables: HashMap<String, VulpineExecutable>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct VulpineExecutable {
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct VulpineDownload {
    pub url: String,
    pub sha256: Option<String>,
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
