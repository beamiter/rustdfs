use serde::{Deserialize, Serialize};
use crate::Command;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Keybind {
    pub command: Command,
    pub modifier: Vec<String>,
    pub key: String,
}
