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

impl LayoutManager {
    pub fn new(config: &impl Config) -> Self {
        let layouts_per_workspaces = config
            .workspaces()
            .unwrap_or_default()
            .iter()
            .map(|ws| {
                (
                    ws.id.unwrap_or_default(),
                    ws.layouts.clone().unwrap_or_default(),
                )
            })
            .collect();
        Self {
            mode: config.layout_mode(),
            layouts: config.layouts(),
            layouts_per_workspaces,
        }
    }

    pub fn new_layout(&self, workspace_id: Option<i32>) -> Layout {
        *self
            .layouts(workspace_id)
            .first()
            .unwrap_or(&Layout::default())
    }

    pub fn next_layout(&self, workspace: &Workspace) -> Layout {
        let layouts = self.layouts(workspace.id);
        let next = match layouts.iter().position(|&x| x == workspace.layout) {
            Some(index) if index == layouts.len() - 1 => layouts.first(),
            Some(index) => layouts.get(index + 1),
            None => None,
        };

        *next.unwrap_or_else(|| layouts.first().unwrap_or(&workspace.layout))
    }

    pub fn previous_layout(&self, workspace: &Workspace) -> Layout {
        let layouts = self.layouts(workspace.id);
        let next = match layouts.iter().position(|&x| x == workspace.layout) {
            Some(index) if index == 0 => layouts.last(),
            Some(index) => layouts.get(index - 1),
            None => None,
        };
        *next.unwrap_or_else(|| layouts.first().unwrap_or(&workspace.layout))
    }

    pub fn update_layouts(
        &self,
        workspaces: &mut Vec<Workspace>,
        mut tags: Vec<&mut Tag>,
    ) -> Option<bool> {
        for workspace in workspaces {
            let tag = tags.iter_mut().find(|t| t.id == workspace.tags[0])?;
            match self.mode {
                LayoutMode::Workspace => {
                    tag.set_layout(workspace.layout, workspace.main_width_percentage);
                }
                LayoutMode::Tag => {
                    workspace.layout = tag.layout;
                    workspace.main_width_percentage = tag.main_width_percentage;
                }
            }
        }
        Some(true)
    }

    fn layouts(&self, workspace_id: Option<i32>) -> &Vec<Layout> {
        workspace_id
            .and_then(|id| self.layouts_per_workspaces.get(&id))
            .and_then(|layouts| {
                if layouts.is_empty() {
                    None
                } else {
                    Some(layouts)
                }
            })
            .unwrap_or(&self.layouts)
    }
}
