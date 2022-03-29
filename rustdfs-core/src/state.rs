use crate::config::{Config, InsertBehavior, ScratchPad};
use crate::layouts::Layout;
use crate::models::{
    FocusManager, LayoutManager, Mode, Screen, Size, Tags, Window, WindowHandle, Workspace,
};
use crate::DisplayAction;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

#[derive(Serialize, Deserialize, Debug)]
pub struct State {
    pub screens: Vec<Screen>,
    pub windows: Vec<Window>,
    pub workspaces: Vec<Workspace>,
    pub focus_manager: FocusManager,
    pub layout_manager: LayoutManager,
    pub mode: Mode,
    pub layouts: Vec<Layout>,
    pub scratchpads: Vec<ScratchPad>,
    pub active_scratchpads: HashMap<String, Option<u32>>,
    pub actions: VecDeque<DisplayAction>,
    pub tags: Tags,
    pub mousekey: Vec<String>,
    pub max_window_width: Option<Size>,
    pub default_width: i32,
    pub default_height: i32,
    pub disable_tile_drag: bool,
    pub insert_behavior: InsertBehavior,
}

impl State {
    pub(crate) fn new(config: &impl Config) -> Self {
        let layout_manager = LayoutManager::new(config);
        let mut tags = Tags::new();
        config.create_list_of_tag_tables().iter().for_each(|label| {
            tags.add_new(label.as_str(), layout_manager.new_layout(None));
        });

        Self {
            focus_manager: FocusManager::new(config),
            layout_manager,
            scratchpads: config.create_list_of_scratchpads(),
            layouts: config.layouts(),
            screens: Default::default(),
            windows: Default::default(),
            workspaces: Default::default(),
            mode: Default::default(),
            active_scratchpads: Default::default(),
            actions: Default::default(),
            tags,
            max_window_width: config.max_window_width(),
            mousekey: config.mousekey(),
            default_width: config.default_width(),
            default_height: config.default_height(),
            disable_tile_drag: config.disable_tile_drag(),
            insert_behavior: config.insert_behavior(),
        }
    }

    pub fn sort_windows(&mut self) {
        use crate::models::WindowType;
        let (level1, other): (Vec<&Window>, Vec<&Window>) = self.windows.iter().partition(|w| {
            w.r#type == WindowType::Dialog
                || w.r#type == WindowType::Splash
                || w.r#type == WindowType::Utility
                || w.r#type == WindowType::Menu
        });

	let (level2, other): (Vec<&Window>, Vec<&Window>) = other
    .iter().partition(|w| w.r#type == WindowType::Normal && w.floating());

	let (level3, other): (Vec<&Window>, Vec<&Window>) =
	    other.iter().partition(|w| w.r#type == WindowType::Normal);

	self.windows = level1
	    .iter().chain(level2.iter()).chain(level3.iter()).chain(other.iter())
    .map(|&w| w.clone()).collect();

	let act = DisplayAction::SetWindowOrder(self.windows.clone());
	self.actions.push_back(act);
    }
}
