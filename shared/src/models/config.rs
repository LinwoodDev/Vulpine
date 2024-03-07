use std::collections::HashMap;

use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct VulpineAppConfig {
    pub executables: HashMap<String, VulpineConfigExecutable>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct VulpineConfigExecutable {
    pub imported: bool,
    pub path: String,
}
