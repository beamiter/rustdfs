use crate::config::Keybind;
use crate::models::TagId;
use crate::models::Window;
use crate::models::WindowHandle;
use crate::models::WindowState;
use crate::utils::xkeysym_lookup::Button;
use serde::{Deserialize, Serialize};

#[allow(clippy::large_enum_variant)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum DisplayAction {
    KillWindow(WindowHandle),

    AddedWindow(WindowHandle, bool, bool),

    MoveMouseOver(WindowHandle, bool),

    MoveMouseOverPoint((i32, i32)),

    SetState(WindowHandle, bool, WindowState),

    SetWindowOrder(Vec<Window>),

    MoveToTop(WindowHandle),

    DestroyedWindow(WindowHandle),

    WindowTakeFocus {
        window: Window,
        previous_window: Option<Window>,
    },

    Unfocus(Option<WindowHandle>, bool),

    FocusWindowUnderCursor,

    ReplayClick(WindowHandle, Button),

    ReadyToResizeWindow(WindowHandle),

    ReadyToMoveWindow(WindowHandle),

    SetCurrentTags(Vec<TagId>),

    SetWindowTags(WindowHandle, Vec<TagId>),

    NormalMode,

    ReloadKeyGrabs(Vec<Keybind>),

    ConfigureXlibWindow(Window),
}
