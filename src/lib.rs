#![allow(dead_code)]

use std::error::Error;

use crate::app::AppBuilderStage1;

pub mod app;
pub mod bind;
pub mod buffer;
pub mod color;
pub mod frame;
pub mod gpu;
pub mod input;
pub mod io;
pub mod material;
pub mod model;
pub mod pipeline;
pub mod prelude;
pub mod shapes;
pub mod texture;
pub mod uniform;
pub mod vertex;

#[derive(thiserror::Error, Debug)]
pub enum LuxError {
    #[error("wgpu failed to be initialized: {error}")]
    WgpuFailedInit { error: String },

    #[error("failed reading file: {0}")]
    IoError(String),

    #[error("issue found while starting app")]
    AppInitFailed,

    #[error("issue found on app's frame")]
    AppFrameFailed,
}

pub fn setup() -> AppBuilderStage1 {
    env_logger::init();
    AppBuilderStage1::default()
}
