use anyhow::{self, Context};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("invalid argument error: {0}")]
    InvalidArgumentError(String),
}
