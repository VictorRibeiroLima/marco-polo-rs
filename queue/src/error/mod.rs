use std::fmt::Display;

use marco_polo_rs_core::SyncError;

#[derive(Debug)]
pub enum HandlerError {
    Retrievable(SyncError),
    Final(SyncError),
}

impl Display for HandlerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HandlerError::Retrievable(error) => write!(f, "{}", error),
            HandlerError::Final(error) => write!(f, "{}", error),
        }
    }
}

impl From<SyncError> for HandlerError {
    fn from(error: SyncError) -> Self {
        HandlerError::Retrievable(error)
    }
}

impl From<sqlx::Error> for HandlerError {
    fn from(error: sqlx::Error) -> Self {
        HandlerError::Retrievable(SyncError::from(error))
    }
}

impl From<std::io::Error> for HandlerError {
    fn from(error: std::io::Error) -> Self {
        HandlerError::Retrievable(SyncError::from(error))
    }
}
