use super::{workspace, TagId};
use crate::{layouts::Layout, models::Window, models::Workspace};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tags {
    normal: Vec<Tag>,
    hidden: Vec<Tag>,
}

impl Tags {
    pub const fn new() -> Self {
        Self {
            normal: vec![],
            hidden: vec![],
        }
    }
    pub fn add_new(&mut self, label: &str, layout: Layout) -> TagId {
        let next_id = self.normal.len() + 1;
        let tag = Tag::new(next_id, label, layout);
        let id = tag.id;
        self.normal.push(tag);
        id
    }
    pub fn add_new_unlabeled(&mut self, layout: Layout) -> TagId {
        let next_id = self.normal.len() + 1;
        self.add_new(next_id.to_string().as_str(), layout)
    }
    pub fn add_new_hidden(&mut self, label: &str) -> Option<TagId> {
        if self.get_hidden_by_label(label).is_none() {
            let next_id = usize::MAX - self.hidden.len();
            let tag = Tag {
                id: next_id,
                label: label.to_string(),
                hidden: true,
                ..Tag::default()
            };
            let id = tag.id;
            self.hidden.push(tag);
            Some(id)
        } else {
            log::error!("tried but failed {}", label);
            None
        }
    }
    pub const fn normal(&self) -> &Vec<Tag> {
        &self.normal
    }
    pub fn all(&self) -> Vec<&Tag> {
        let mut result: Vec<&Tag> = vec![];
        self.normal.iter().for_each(|tag| result.push(tag));
        self.hidden.iter().for_each(|tag| result.push(tag));
        result
    }
    pub fn all_mut(&mut self) -> Vec<&mut Tag> {
        let mut result: Vec<&mut Tag> = vec![];
        self.normal.iter_mut().for_each(|tag| result.push(tag));
        self.hidden.iter_mut().for_each(|tag| result.push(tag));
        result
    }
    pub fn get(&self, id: TagId) -> Option<&Tag> {
        self.normal.get(id - 1).or_else(|| 
        self.hidden.iter().find(|&hidden_tag| hidden_tag.id == id))
    }
    pub fn get_mut(&mut self, id: TagId) -> Option<&mut Tag> {
        if let Some(normal) = self.normal.get_mut(id - 1) {
            return Some(normal);
        }
        return self.hidden.iter_mut().find(|hidden_tag| hidden_tag.id == id);
    }
    pub fn get_hidden_by_label(&self, label: &str) -> Option<&Tag> {
        self.hidden.iter().find(|tag| tag.label.eq(label))
    }
    pub fn len_normal(&self) -> usize {
        self.normal.len()
    }
}

impl Default for Tags {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Tag {
    pub id: TagId,
    pub label: String,
    pub hidden: bool,
    pub layout: Layout,
    pub main_width_percentage: u8,
    pub flipped_horizontal: bool,
    pub flipped_vertical: bool,
    pub layout_rotation: usize,
}

impl Tag {
    #[must_use]
    pub fn new(id: TagId, label: &str, layout: Layout) -> Self {
        Self {
            id,
            label: label.to_owned(),
            hidden: false,
            layout,
            main_width_percentage: layout.main_width(),
            flipped_horizontal: false,
            flipped_vertical: false,
            layout_rotation: 0,
        }
    }
    pub fn update_windows(&self, windows: &mut [Window], workspace: &Workspace) {
        if let Some(window) = windows
            .iter_mut()
            .find(|w| w.has_tag(&self.id) && w.is_fullscreen())
        {
            window.set_visible(true);
            window.normal = workspace.xyhw;
            let handle = window.handle;
            windows
                .iter_mut()
                .filter(|w| {
                    w.has_tag(&self.id)
                        && w.transient.unwrap_or_else(|| 0.into()) == handle
                        && !w.is_unmanaged()
                })
                .for_each(|w| {
                    w.set_visible(true);
                });
        } else {
            let mut all_mine: Vec<&mut Window> =
                windows.iter_mut().filter(|w| w.has_tag(&self.id)).collect();
            all_mine.iter_mut().for_each(|w| w.set_visible(true));
            let mut managed_nonfloat: Vec<&mut Window> = windows
                .iter_mut()
                .filter(|w| w.has_tag(&self.id) && !w.is_unmanaged())
                .collect();
            self.layout
                .update_windows(workspace, &mut managed_nonfloat, self);
            for w in &mut managed_nonfloat {
                w.container_size = Some(workspace.xyhw);
            }
            windows
                .iter_mut()
                .filter(|w| w.has_tag(&self.id) && !w.is_unmanaged() && w.floating())
                .for_each(|w| w.normal = workspace.xyhw);
        }
    }

    pub fn change_main_width(&mut self, delta: i8) {
        self.main_width_percentage = (self.main_width_percentage as i8 + delta).max(0).min(100) as u8;
    }
    pub fn set_main_width(&mut self, val: u8) {
        self.main_width_percentage = val.min(100);
    }
    #[must_use]
    pub fn main_width_percentage(&self) -> f32 {
        f32::from(self.main_width_percentage)
    }
    pub fn set_layout(&mut self, layout: Layout, main_width_percentage: u8) {
        self.layout = layout;
        self.set_main_width(main_width_percentage);
        self.layout_rotation = 0;
    }
    pub fn rotate_layout(&mut self) -> Option<()> {
        let rotations = self.layout.rotations();
        self.layout_rotation += 1;
        if self.layout_rotation >= rotations.len() {
            self.layout_rotation = 0;
        }
        let (horz, vert) = rotations.get(self.layout_rotation)?;
        self.flipped_horizontal = *horz;
        self.flipped_vertical = *vert;
        Some(())
    }
}
