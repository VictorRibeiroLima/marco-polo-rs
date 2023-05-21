pub mod database;
pub mod env;
pub mod internals;
pub mod util;

pub type SyncError = Box<dyn std::error::Error + Send + Sync + 'static>;
