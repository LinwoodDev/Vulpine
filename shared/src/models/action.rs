use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::app::ActionInput;

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct ActionDisplay {
    pub args: HashMap<String, ActionInput>,
    pub inputs: HashMap<String, ConnectorType>,
    pub outputs: HashMap<String, ConnectorType>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ConnectorType {
    Flow,
    String,
    Integer,
    Float,
    Boolean,
}
