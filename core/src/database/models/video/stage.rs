use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

#[derive(Debug, Serialize, Clone, PartialEq, Deserialize, sqlx::Type)]
#[sqlx(type_name = "videos_video_stages", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VideoStage {
    Downloading,
    Cutting,
    RawUploading,
    Transcribing,
    Translating,
    Subtitling,
    Uploading,
    Done,
}

impl Display for VideoStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VideoStage::Downloading => write!(f, "Downloading"),
            VideoStage::Cutting => write!(f, "Cutting"),
            VideoStage::RawUploading => write!(f, "RawUploading"),
            VideoStage::Transcribing => write!(f, "Transcribing"),
            VideoStage::Translating => write!(f, "Translating"),
            VideoStage::Subtitling => write!(f, "Subtitling"),
            VideoStage::Uploading => write!(f, "Uploading"),
            VideoStage::Done => write!(f, "Done"),
        }
    }
}

impl FromStr for VideoStage {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Downloading" => Ok(VideoStage::Downloading),
            "Cutting" => Ok(VideoStage::Cutting),
            "RawUploading" => Ok(VideoStage::RawUploading),
            "Transcribing" => Ok(VideoStage::Transcribing),
            "Translating" => Ok(VideoStage::Translating),
            "Subtitling" => Ok(VideoStage::Subtitling),
            "Uploading" => Ok(VideoStage::Uploading),
            "Done" => Ok(VideoStage::Done),
            _ => Err(format!(
                "{} is not a valid video stage. expected ('Downloading', 'Transcribing', 'Translating', 'Subtitling', 'Uploading', 'Done')",
                s
            )),
        }
    }
}
