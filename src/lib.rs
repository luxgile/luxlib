#![allow(dead_code)]

use crate::app::AppBuilder;

mod pipeline;
mod buffer;
mod vertex;
mod gpu;
mod app;

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
