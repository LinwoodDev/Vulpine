use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
enum VulpineAction {
    Command {
        executable: String,
        inputs: HashMap<String, ActionInput>,
        args: Vec<HashMap<String, ValueMapping>>,
        env: HashMap<String, ValueMapping>,
    }
}

#[derive(Debug, Serialize, Deserialize)]
enum ActionInput {
    Checkbox {
        label: String,
    }
}

#[derive(Debug, Serialize, Deserialize)]
enum ValueMapping {
    Constant {
        value: String,
    },
    Input {
        input: String,
        mapping: HashMap<String, String>,
    }
}
