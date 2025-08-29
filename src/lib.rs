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
pub mod material;
pub mod uniform;
pub mod io;

type LResult<T> = Result<T, LuxError>;
#[derive(thiserror::Error, Debug)]
pub enum LuxError {
    #[error("wgpu failed to be initialized: {error}")]
    WgpuFailedInit { error: String },

    #[error("failed reading file: {0}")]
    IoError(String),
}

pub fn setup() -> AppBuilderStage1 {
    env_logger::init();
    AppBuilderStage1::default()
}

