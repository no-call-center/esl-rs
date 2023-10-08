use std::num::ParseIntError;

use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Ord, PartialOrd, Eq, Hash, Error)]
pub enum EslError {
    #[error("unknown error")]
    InternalError(String),

    #[error("Wrong password.")]
    AuthFailed,

    #[error("Acl rejected.")]
    AclRejected,

    #[error("Unable to connect to destination server.")]
    ConnectionError,

    #[error("empty event")]
    EmptyEvent,

    #[error("{0:?}")]
    ApiError(String),

    #[error("")]
    CodeParseError(),

    #[error("Didnt get any digits")]
    NoInput,
}

pub type Result<T> = std::result::Result<T, EslError>;

impl From<std::io::Error> for EslError {
    fn from(error: std::io::Error) -> Self {
        Self::InternalError(error.to_string())
    }
}
impl From<tokio::sync::oneshot::error::RecvError> for EslError {
    fn from(error: tokio::sync::oneshot::error::RecvError) -> Self {
        Self::InternalError(error.to_string())
    }
}

impl From<tokio::sync::mpsc::error::TryRecvError> for EslError {
    fn from(error: tokio::sync::mpsc::error::TryRecvError) -> Self {
        match error {
            tokio::sync::mpsc::error::TryRecvError::Empty => Self::EmptyEvent,
            tokio::sync::mpsc::error::TryRecvError::Disconnected => Self::ConnectionError,
        }
    }
}

impl From<serde_json::Error> for EslError {
    fn from(error: serde_json::Error) -> Self {
        Self::InternalError(error.to_string())
    }
}
impl From<ParseIntError> for EslError {
    fn from(error: ParseIntError) -> Self {
        Self::InternalError(error.to_string())
    }
}
