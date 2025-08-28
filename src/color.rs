use bytemuck::{Pod, Zeroable};

use crate::bind::BindEntry;

#[repr(C)]
#[derive(Clone, Copy, Default, Debug, Pod, Zeroable)]
pub struct Srgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
impl Srgba {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub const CLEAR: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };

    pub const RED: Self = Self {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };

    pub const WHITE: Self = Self {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };

    pub const BLACK: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };

    pub const BLUE: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };

    pub const GREEN: Self = Self {
        r: 0.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };

    pub const YELLOW: Self = Self {
        r: 1.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };

    pub const SILVER: Self = Self {
        r: 0.75,
        g: 0.75,
        b: 0.75,
        a: 1.0,
    };

    pub const GRAY: Self = Self {
        r: 0.5,
        g: 0.5,
        b: 0.5,
        a: 1.0,
    };

    pub const DARK_GRAY: Self = Self {
        r: 0.3,
        g: 0.3,
        b: 0.3,
        a: 1.0,
    };

    pub const ORANGE: Self = Self {
        r: 1.0,
        g: 0.65,
        b: 0.0,
        a: 1.0,
    };

    pub const PURPLE: Self = Self {
        r: 0.5,
        g: 0.0,
        b: 0.5,
        a: 1.0,
    };

    pub const MAGENTA: Self = Self {
        r: 1.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };

    pub const CYAN: Self = Self {
        r: 0.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };

    pub const PINK: Self = Self {
        r: 1.0,
        g: 0.75,
        b: 0.8,
        a: 1.0,
    };

    pub const TEAL: Self = Self {
        r: 0.0,
        g: 0.5,
        b: 0.5,
        a: 1.0,
    };

    pub const OLIVE: Self = Self {
        r: 0.5,
        g: 0.5,
        b: 0.0,
        a: 1.0,
    };

    pub const MAROON: Self = Self {
        r: 0.5,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };

    pub const NAVY: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.5,
        a: 1.0,
    };

    pub const GOLD: Self = Self {
        r: 1.0,
        g: 0.84,
        b: 0.0,
        a: 1.0,
    };
}
