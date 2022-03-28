use super::{models::Screen, models::Window, models::WindowHandle, Button, ModMask,
XKeysym};

use crate::models::WindowChange;
use crate::Command;

#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum DisplayEvent {
    Movement(WindowHandle, i32, i32),
    KeyCombo(ModMask, XKeysym),
    KeyGrapReload,
    MouseCombo(ModMask, Button, WindowHandle, i32, i32),
    WindowCreate(Window, i32, i32),
    WindowChange(WindowChange),
    WindowDestroy(WindowHandle),
    WindowTakeFocus(WindowHandle),
    HandleWindowFocus(WindowHandle),
    VerifyFocusedAt(WindowHandle),
    MoveFocusTo(i32, i32),
    MoveWindow(WindowHandle, i32, i32),
    ResizeWindow(WindowHandle, i32, i32),
    ScreenCreate(Screen),
    SendCommand(Command),
    ConfigureXlibWindow(WindowHandle),
    ChangeToNormalMode,
}
