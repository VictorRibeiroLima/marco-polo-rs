use lazy_static::lazy_static;
use marco_polo_rs_core::database::models::video_storage::VideoFormat;
use regex::Regex;
use serde::{Deserialize, Serialize};

use validator::{Validate, ValidationError};

use marco_polo_rs_core::MAX_NUMBER_OF_CUTS;

lazy_static! {
    static ref YOUTUBE_URL: Regex = Regex::new(r#"^((?:https?:)?//)?((?:www|m)\.)?((?:youtube\.com|youtu.be))(/(?:[\w\-]+\?v=|embed/|v/)?)([\w\-]+)(\S+)?$"#).unwrap();
}

fn validate_time(time: &str) -> Result<(), ValidationError> {
    let times = time.split(":").collect::<Vec<&str>>();
    if times.len() != 3 {
        return Err(ValidationError::new("Invalid Time Format"));
    }

    for time in times {
        time.parse::<i32>()
            .map_err(|_| ValidationError::new("Invalid Time Format"))?;
    }
    return Ok(());
}

#[derive(Debug, Default, Validate, Deserialize, Serialize, Clone)]
pub struct Create {
    #[validate(regex(path = "YOUTUBE_URL", message = "Invalid Youtube URL"))]
    pub video_url: String,
    pub language: Option<String>,
    pub format: Option<VideoFormat>,
    #[validate]
    #[validate(length(min = 1, max = "MAX_NUMBER_OF_CUTS"))]
    pub cuts: Vec<Cut>,
}

#[derive(Debug, Default, Validate, Deserialize, Serialize, Clone)]
pub struct Cut {
    pub title: String,
    pub description: String,
    pub channel_ids: Vec<i32>,
    #[validate(custom(
        function = "validate_time",
        message = "Invalid Time Format (HH:MM:SS)\n"
    ))]
    pub start_time: Option<String>,
    #[validate(custom(
        function = "validate_time",
        message = "Invalid Time Format (HH:MM:SS)\n"
    ))]
    pub end_time: Option<String>,
    pub tags: Option<Vec<String>>,
}
