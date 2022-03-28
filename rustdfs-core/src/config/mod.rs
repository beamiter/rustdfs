mod keybind;
mod workspace_config;
mod scratchpad;
mod insert_behavior;

pub use insert_behavior::InsertBehavior;
pub use keybind::Keybind;
pub use scratchpad::ScratchPad;
pub use workspace_config::Workspace;
pub use crate::models::{FocusBehaviour, Gutter, Margins, Size};

use crate::layouts::Layout;
use crate::display_servers::DisplayServer;
use crate::models::{Window, WindowType, LayoutMode};

pub trait Config {
    fn mapped_bindings(&self) -> Vec<Keybind>;
    fn create_list_of_tag_tables(&self) -> Vec<String>;
    fn workspaces(&self) -> Option<Vec<Workspace>>;
    fn focus_behavior(&self) -> FocusBehaviour;
    fn mousekey(&self) -> Vec<String>;
    fn create_list_of_scratchpads(&self) -> Vec<ScratchPad>;
    fn layouts(&self) -> Vec<Layout>;
    fn layout_mode(&self) -> LayoutMode;
    fn focus_new_windows(&self) -> bool;

    fn always_float(&self) -> bool;
    fn default_width(&self) -> i32;
    fn default_height(&self) -> i32;
    fn border_width(&self) -> i32;
    fn margin(&self) -> Margins;
    fn workspace_margin(&self) -> Option<Margins>;
    fn gutter(&self) -> Option<Vec<Gutter>>;
    fn default_border_color(&self) -> String;
    fn floating_border_color(&self) -> String;
    fn on_new_window_cmd(&self) -> Option<String>;
    fn get_list_of_gutters(&self) -> Vec<Gutter>;
    fn max_window_width(&self) -> Option<Size>;
    fn disable_tile_drag(&self) -> bool;

    //fn save_state(&self, state: &State);
    //fn load_state(&self, state: &mut State);
    fn setup_predefined_window(&self, window: &mut Window) -> bool;
    fn load_window(&self, window: &mut Window) {
       if window.r#type == WindowType::Normal {
           window.margin = self.margin();
           window.border = self.border_width();
           window.must_float = self.always_float();
       } else {
           window.margin = Margins::new(0);
           window.border = 0;
       }
    }
}
