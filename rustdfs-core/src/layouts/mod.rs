use serde::{Deserialize, Serialize};
use std::str::FromStr;
use thiserror::Error;
use super::models::Workspace;
use crate::models::Tag;
use super::models::Window;

mod center_main;
mod center_main_balanced;
mod even_horizontal;
mod even_vertical;
mod fibonacci;
mod grid_horizontal;
mod main_and_deck;
mod main_and_horizontal_stack;
mod main_and_vert_stack;
mod monocle;
mod right_main_and_vert_stack;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Layout {
    MainAndVertStack,
    MainAndHorizontalStack,
    MainAndDeck,
    GridHorizontal,
    EventHorizontal,
    EventVertical,
    Fibonacci,
    CenterMain,
    CenterMainBalanced,
    Monocle,
    RightWiderLeftStack,
    LeftWiderRightStack,
}

pub const LAYOUTS: &[Layout] = &[
    Layout::MainAndVertStack,
    Layout::MainAndHorizontalStack,
    Layout::MainAndDeck,
    Layout::GridHorizontal,
    Layout::EventHorizontal,
    Layout::EventVertical,
    Layout::Fibonacci,
    Layout::CenterMain,
    Layout::CenterMainBalanced,
    Layout::Monocle,
    Layout::RightWiderLeftStack,
    Layout::LeftWiderRightStack,
];

impl Default for Layout {
    fn default() -> Self {
        Self::MainAndVertStack
    }
}

impl Layout {
    pub fn update_windows(&self, workspace: &Workspace, windows: &mut [&mut Window],
                          tag: &Tag) {
        match self {
            Self::MainAndVertStack | Self::LeftWiderRightStack => {
                main_and_vert_stack::update(workspace, tag, windows);
            }
        }
    }

    pub const fn main_width(&self) -> u8 {
        match self {
            Self::RightWiderLeftStack | Self::LeftWiderRightStack => 75,
            _ => 50,
        }
    }

    pub fn rotations(&self) -> Vec<(bool, bool)> {
        match self {
            Self::Fibonacci | Self::GridHorizontal => {
                [(false, false), (true, false), (true, true), (false, true)].to_vec()
            }
            Self::MainAndHorizontalStack => [(false, false), (false, true)].to_vec(),
            _ => [(false, false), (true, false)].to_vec(),
        }
    }
}
