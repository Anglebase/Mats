mod matrix;
mod ops;
mod math;

pub use matrix::*;

#[derive(Debug, Clone, thiserror::Error)]
pub enum MatrixError {
    #[error("Matrix dimensions do not match: expected {expected}, actual {actual}")]
    DimensionsNotMatch { expected: String, actual: String },
}

pub type Result<T> = std::result::Result<T, MatrixError>;