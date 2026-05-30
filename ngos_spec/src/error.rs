
use thiserror::Error;

pub const NGOS_SUCCESS: i32 = 0;
pub const NGOS_ERROR_INVALID_ARGUMENT: i32 = 1;
pub const NGOS_ERROR_BUFFER_TOO_SMALL: i32 = 2;
pub const NGOS_ERROR_UNSUPPORTED: i32 = 3;
pub const NGOS_ERROR_PLUGIN: i32 = 4;
pub const NGOS_ERROR_INTERNAL: i32 = 5;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum NgosError {
    #[error("invalid argument: {0}")]
    InvalidArgument(String),
    #[error("buffer too small")]
    BufferTooSmall,
    #[error("unsupported: {0}")]
    Unsupported(String),
    #[error("plugin error: {0}")]
    Plugin(String),
    #[error("internal error: {0}")]
    Internal(String),
}

pub type NgosResult<T> = Result<T, NgosError>;

pub fn status_to_result(status: i32) -> NgosResult<()> {
    match status {
        NGOS_SUCCESS => Ok(()),
        NGOS_ERROR_INVALID_ARGUMENT => Err(NgosError::InvalidArgument(
            "plugin rejected argument".into(),
        )),
        NGOS_ERROR_BUFFER_TOO_SMALL => Err(NgosError::BufferTooSmall),
        NGOS_ERROR_UNSUPPORTED => Err(NgosError::Unsupported(
            "plugin capability unavailable".into(),
        )),
        NGOS_ERROR_PLUGIN => Err(NgosError::Plugin("plugin returned failure".into())),
        _ => Err(NgosError::Internal(format!("unknown status code {status}"))),
    }
}

pub fn error_to_status(error: &NgosError) -> i32 {
    match error {
        NgosError::InvalidArgument(_) => NGOS_ERROR_INVALID_ARGUMENT,
        NgosError::BufferTooSmall => NGOS_ERROR_BUFFER_TOO_SMALL,
        NgosError::Unsupported(_) => NGOS_ERROR_UNSUPPORTED,
        NgosError::Plugin(_) => NGOS_ERROR_PLUGIN,
        NgosError::Internal(_) => NGOS_ERROR_INTERNAL,
    }
}

