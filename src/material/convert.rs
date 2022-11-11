use crate::{
    color::Color,
    pattern::{Checker, Gradient, Patterns, Ring, Stripe},
};

use super::Texture;

impl From<Color> for Texture {
    fn from(c: Color) -> Self {
        Self::Color(c)
    }
}

impl From<Checker> for Texture {
    fn from(c: Checker) -> Self {
        Self::Pattern(Patterns::Checker(c))
    }
}

impl From<Gradient> for Texture {
    fn from(g: Gradient) -> Self {
        Self::Pattern(Patterns::Gradient(g))
    }
}

impl From<Ring> for Texture {
    fn from(r: Ring) -> Self {
        Self::Pattern(Patterns::Ring(r))
    }
}

impl From<Stripe> for Texture {
    fn from(s: Stripe) -> Self {
        Self::Pattern(Patterns::Stripe(s))
    }
}
