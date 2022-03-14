use super::Tag;
use crate::{config::Config, layouts::Layout, models::Workspace};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum LayoutMode {
    Tag,
    Workspace,
}

impl Default for LayoutMode {
    fn default() -> Self {
        Self::Workspace
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LayoutManager {
    pub mode: LayoutMode,
    pub layouts: Vec<Layout>,
    pub layouts_per_workspaces: HashMap<i32, Vec<Layout>>,
}
