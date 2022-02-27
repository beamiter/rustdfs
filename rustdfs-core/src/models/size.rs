use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy)]
#[serde(untagged)]
pub enum Size {
    Pixel(i32),
    Ratio(f32),
}

impl Size {
    #[must_use]
    pub fn into_absolute(self, whole: i32) -> i32 {
        match self {
            Size::Pixel(x) => x,
            Size::Ratio(x) => (whole as f32 * x).floor() as i32,
        }
    }
}
