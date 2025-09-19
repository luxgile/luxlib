use glam::Vec2;

use crate::{
    color::Srgba,
    gpu::{DrawCommand, Gpu, RenderContext},
};

#[derive(Default, Debug, Clone)]
pub struct Rect {
    pub size: Vec2,
}
impl Rect {
    pub fn new(size: Vec2) -> Self {
        Self { size }
    }

    pub fn from_xy(width: f32, height: f32) -> Self {
        Self::new(Vec2::new(width, height))
    }
}


#[derive(Default, Clone, Debug)]
pub struct Circle {
    pub radius: f32,
}
