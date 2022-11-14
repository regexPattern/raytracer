use crate::{color::Color, pattern::Pattern};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Texture {
    Color(Color),
    Pattern(Pattern),
}
