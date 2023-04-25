use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppResult<T> {
    pub data: T,
}

impl<T> AppResult<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}
