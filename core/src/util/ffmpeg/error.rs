use std::fmt::Display;

#[derive(Debug)]
pub enum FfmpegError {
    ParseError(String),
    IoError(std::io::Error),
    ProbeError(String),
    CutError,
}

impl From<serde_json::Error> for FfmpegError {
    fn from(err: serde_json::Error) -> Self {
        FfmpegError::ParseError(err.to_string())
    }
}

impl From<std::io::Error> for FfmpegError {
    fn from(err: std::io::Error) -> Self {
        FfmpegError::IoError(err)
    }
}

impl Display for FfmpegError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FfmpegError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            FfmpegError::IoError(err) => write!(f, "IO error: {}", err),
            FfmpegError::ProbeError(msg) => write!(f, "Probe error: {}", msg),
            FfmpegError::CutError => write!(f, "Failed to cut video"),
        }
    }
}

impl std::error::Error for FfmpegError {}
