mod xyhw;
mod window_state;
mod window_type;
mod margins;
mod window;
mod xyhw_change;
mod window_change;
mod mode;
mod gutter;
mod size;
mod screen;
mod workspace;
mod tag;
mod dock_area;
mod focus_manager;
mod layout_manager;

use crate::layouts;

pub use dock_area::DockArea;
pub use window_state::WindowState;
pub use window_type::WindowType;
pub use window::WindowHandle;
pub use window::Window;
pub use margins::Margins;
pub use mode::Mode;
pub use xyhw::Xyhw;
pub use xyhw::XyhwBuilder;
pub use xyhw_change::XyhwChange;
pub use gutter::Side;
pub use gutter::Gutter;
pub use size::Size;
pub use window_change::WindowChange;
pub use screen::{BBox, Screen};
pub use tag::Tag;
pub use tag::Tags;
pub use workspace::Workspace;
pub use focus_manager::FocusBehaviour;
pub use focus_manager::FocusManager;
pub use layout_manager::LayoutMode;
pub use layout_manager::LayoutManager;

pub type TagId = usize;
type MaybeWindowHandle = Option<WindowHandle>;
