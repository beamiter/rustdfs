use crate::models::Size;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ScratchPad {
    pub name: String,
    pub value: String,
    pub x: Option<Size>,
    pub y: Option<Size>,
    pub height: Option<Size>,
    pub width: Option<Size>,
}
