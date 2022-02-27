use crate::models::Xyhw;
use crate::models::XyhwBuilder;

use super::Screen;

#[derive(Copy, Clone, Debug, Default)]
pub struct DockArea {
    pub top: i32,
    pub top_start_x: i32,
    pub top_end_x: i32,

    pub bottom: i32,
    pub bottom_start_x: i32,
    pub bottom_end_x: i32,

    pub right: i32,
    pub right_start_y: i32,
    pub right_end_y: i32,

    pub left: i32,
    pub left_start_y: i32,
    pub left_end_y: i32,
}

impl From<&[i64]> for DockArea {
    fn from(slice: &[i64]) -> Self {
        Self {
            left: slice[0] as i32,
            right: slice[1] as i32,
            top: slice[2] as i32,
            bottom: slice[3] as i32,
            left_start_y: slice[4] as i32,
            left_end_y: slice[5] as i32,
            right_start_y: slice[6] as i32,
            right_end_y: slice[7] as i32,
            top_start_x: slice[8] as i32,
            top_end_x: slice[9] as i32,
            bottom_start_x: slice[10] as i32,
            bottom_end_x: slice[11] as i32,
        }
    }
}

impl From<&[i32]> for DockArea {
    fn from(slice: &[i32]) -> Self {
        Self {
            left: slice[0],
            right: slice[1],
            top: slice[2],
            bottom: slice[3],
            left_start_y: slice[4],
            left_end_y: slice[5],
            right_start_y: slice[6],
            right_end_y: slice[7],
            top_start_x: slice[8],
            top_end_x: slice[9],
            bottom_start_x: slice[10],
            bottom_end_x: slice[11],
        }
    }
}
