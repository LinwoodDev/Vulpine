use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct VulpineApp {
    pub name: String,
    pub description: String,
    pub version: String,
    pub actions: BTreeMap<String, VulpineAction>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum VulpineAction {
    Command {
        executable: String,
        inputs: HashMap<String, ActionInput>,
        args: Vec<HashMap<String, ValueMapping>>,
        env: HashMap<String, ValueMapping>,
    }
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
