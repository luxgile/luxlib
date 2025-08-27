#![allow(dead_code)]

use crate::app::AppBuilder;

pub mod pipeline;
pub mod buffer;
pub mod vertex;
pub mod gpu;
pub mod app;
pub mod color;
pub mod prelude;
pub mod model;

type LResult<T> = Result<T, LuxError>;
#[derive(thiserror::Error, Debug)]
pub enum LuxError {
    #[error("wgpu failed to be initialized")]
    WgpuFailedInit { error: String },
}


pub fn init() -> AppBuilder {
    env_logger::init();
    AppBuilder::default()
}
