pub use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

// an empty struct containing no data, used as a tag to identify the player entity
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;