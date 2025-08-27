#![allow(dead_code)]

use crate::app::{AppBuilderStage1, AppBuilderStage2, InitFn};

pub mod app;
pub mod buffer;
pub mod color;
pub mod gpu;
pub mod model;
pub mod pipeline;
pub mod prelude;
pub mod vertex;
pub mod frame;
pub mod input;
pub mod shapes;
pub mod texture;
pub mod bind;

type LResult<T> = Result<T, LuxError>;
#[derive(thiserror::Error, Debug)]
pub enum LuxError {
    #[error("wgpu failed to be initialized")]
    WgpuFailedInit { error: String },
}

pub fn setup() -> AppBuilderStage1 {
    env_logger::init();
    AppBuilderStage1::default()
}

