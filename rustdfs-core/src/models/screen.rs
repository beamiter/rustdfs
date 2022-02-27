use serde::{Deserialize, Serialize};
use std::convert::From;
use x11_dl::xlib;
use super::{Size, WindowHandle};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Screen {
    pub root: WindowHandle,
    #[serde(flatten)]
    pub bbox: BBox,
    pub wsid: Option<i32>,
    pub max_window_width: Option<Size>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct BBox {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}
