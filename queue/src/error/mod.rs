use marco_polo_rs_core::SyncError;

#[derive(Debug)]
pub enum HandlerError {
    Retrievable(SyncError),
    Final(SyncError),
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
