use std::{fmt::Display, str::FromStr};

use super::error::FfmpegError;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Time {
    pub hours: i8,
    pub minutes: i8,
    pub seconds: i8,
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02}:{:02}:{:02}",
            self.hours, self.minutes, self.seconds
        )
    }
}

impl FromStr for Time {
    type Err = FfmpegError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(":");

        let hours = match parts
            .next()
            .ok_or_else(|| FfmpegError::ParseError("Failed to parse hours".to_string()))?
            .parse::<i8>()
        {
            Ok(hours) => Ok(hours),
            Err(_) => Err(FfmpegError::ParseError("Failed to parse hours".to_string())),
        }?;

        let minutes = match parts
            .next()
            .ok_or_else(|| FfmpegError::ParseError("Failed to parse minutes".to_string()))?
            .parse::<i8>()
        {
            Ok(minutes) => Ok(minutes),
            Err(_) => Err(FfmpegError::ParseError(
                "Failed to parse minutes".to_string(),
            )),
        }?;

        let seconds = match parts
            .next()
            .ok_or_else(|| FfmpegError::ParseError("Failed to parse seconds".to_string()))?
            .parse::<i8>()
        {
            Ok(seconds) => Ok(seconds),
            Err(_) => Err(FfmpegError::ParseError(
                "Failed to parse seconds".to_string(),
            )),
        }?;

        Ok(Time {
            hours,
            minutes,
            seconds,
        })
    }
}
