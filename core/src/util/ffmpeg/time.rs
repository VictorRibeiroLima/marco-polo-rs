use std::{fmt::Display, str::FromStr};

use super::error::FfmpegError;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Time {
    pub hours: i8,
    pub minutes: i8,
    pub seconds: i8,
}

impl serde::Serialize for Time {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for Time {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let time = String::deserialize(deserializer)?;

        match Time::from_str(&time) {
            Ok(time) => Ok(time),
            Err(e) => Err(serde::de::Error::custom(e.to_string())),
        }
    }
}

impl Time {
    pub fn remove_seconds(&mut self, seconds: i8) {
        self.seconds -= seconds;

        if self.seconds < 0 {
            self.seconds += 60;
            self.minutes -= 1;
        }

        if self.minutes < 0 {
            self.minutes += 60;
            self.hours -= 1;
        }

        if self.hours < 0 {
            self.hours = 0;
            self.minutes = 0;
            self.seconds = 0;
        }
    }

    pub fn to_seconds(&self) -> i64 {
        (self.hours as i64 * 60 * 60) + (self.minutes as i64 * 60) + self.seconds as i64
    }
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

        let mut hours = match parts
            .next()
            .ok_or_else(|| FfmpegError::ParseError("Failed to parse hours".to_string()))?
            .parse::<i8>()
        {
            Ok(hours) => Ok(hours),
            Err(_) => Err(FfmpegError::ParseError("Failed to parse hours".to_string())),
        }?;

        let mut minutes = match parts
            .next()
            .ok_or_else(|| FfmpegError::ParseError("Failed to parse minutes".to_string()))?
            .parse::<i8>()
        {
            Ok(minutes) => Ok(minutes),
            Err(_) => Err(FfmpegError::ParseError(
                "Failed to parse minutes".to_string(),
            )),
        }?;

        let mut seconds = match parts
            .next()
            .ok_or_else(|| FfmpegError::ParseError("Failed to parse seconds".to_string()))?
            .parse::<i8>()
        {
            Ok(seconds) => Ok(seconds),
            Err(_) => Err(FfmpegError::ParseError(
                "Failed to parse seconds".to_string(),
            )),
        }?;

        // We need to check minutes 2 times to make sure we don't overflow.
        //"seconds" will not overflow because we never add to it. so any value greater than i8::MAX will error before we get here.
        if minutes > 60 {
            hours = hours
                .checked_add(minutes / 60)
                .ok_or(FfmpegError::ParseError(
                    "Hours cannot be greater than 127".to_string(),
                ))?;
            minutes %= 60;
        }

        if seconds > 60 {
            minutes += seconds / 60;
            seconds %= 60;
        }

        // second time checking minutes to make sure we didn't overflow.
        if minutes > 60 {
            println!("minutes: {}", minutes);
            hours = hours
                .checked_add(minutes / 60)
                .ok_or(FfmpegError::ParseError(
                    "Hours cannot be greater than 127".to_string(),
                ))?;
            minutes %= 60;
        }

        Ok(Time {
            hours,
            minutes,
            seconds,
        })
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::util::ffmpeg::time::Time;

    #[test]
    fn test_time_comparison() {
        let time1 = Time {
            hours: 0,
            minutes: 0,
            seconds: 0,
        };

        let time2 = Time {
            hours: 0,
            minutes: 0,
            seconds: 1,
        };

        let time3 = Time {
            hours: 0,
            minutes: 1,
            seconds: 0,
        };

        let time4 = Time {
            hours: 1,
            minutes: 0,
            seconds: 0,
        };

        let time5 = Time {
            hours: 1,
            minutes: 0,
            seconds: 59,
        };

        let time6 = Time {
            hours: 1,
            minutes: 1,
            seconds: 0,
        };

        assert!(time1 < time2);

        assert!(time1 < time3);
        assert!(time2 < time3);
        assert!(time1 < time4);
        assert!(time2 < time4);
        assert!(time3 < time4);

        assert!(time1 < time5);
        assert!(time2 < time5);
        assert!(time3 < time5);
        assert!(time4 < time5);

        assert!(time1 < time6);
        assert!(time2 < time6);
        assert!(time3 < time6);
        assert!(time4 < time6);
        assert!(time5 < time6);
    }

    #[test]
    fn test_time_to_seconds() {
        let time1 = Time {
            hours: 0,
            minutes: 0,
            seconds: 0,
        };

        let time2 = Time {
            hours: 0,
            minutes: 0,
            seconds: 1,
        };

        let time3 = Time {
            hours: 0,
            minutes: 1,
            seconds: 0,
        };

        let time4 = Time {
            hours: 1,
            minutes: 0,
            seconds: 0,
        };

        let time5 = Time {
            hours: 1,
            minutes: 0,
            seconds: 59,
        };

        let time6 = Time {
            hours: 1,
            minutes: 1,
            seconds: 0,
        };

        assert_eq!(time1.to_seconds(), 0);
        assert_eq!(time2.to_seconds(), 1);
        assert_eq!(time3.to_seconds(), 60);
        assert_eq!(time4.to_seconds(), 60 * 60);
        assert_eq!(time5.to_seconds(), 60 * 60 + 59);
        assert_eq!(time6.to_seconds(), 60 * 60 + 60);
    }

    #[test]
    fn test_time_remove_seconds() {
        let mut time1 = Time {
            hours: 0,
            minutes: 0,
            seconds: 0,
        };

        let mut time2 = Time {
            hours: 0,
            minutes: 0,
            seconds: 1,
        };

        let mut time3 = Time {
            hours: 0,
            minutes: 1,
            seconds: 0,
        };

        let mut time4 = Time {
            hours: 1,
            minutes: 0,
            seconds: 0,
        };

        let mut time5 = Time {
            hours: 1,
            minutes: 0,
            seconds: 59,
        };

        let mut time6 = Time {
            hours: 1,
            minutes: 1,
            seconds: 0,
        };

        time1.remove_seconds(1);
        time2.remove_seconds(1);
        time3.remove_seconds(1);
        time4.remove_seconds(1);
        time5.remove_seconds(1);
        time6.remove_seconds(1);

        assert_eq!(time1, Time::from_str("00:00:00").unwrap());
        assert_eq!(time2, Time::from_str("00:00:00").unwrap());
        assert_eq!(time3, Time::from_str("00:00:59").unwrap());
        assert_eq!(time4, Time::from_str("00:59:59").unwrap());
        assert_eq!(time5, Time::from_str("01:00:58").unwrap());
        assert_eq!(time6, Time::from_str("01:00:59").unwrap());
    }

    #[test]
    fn test_from_str() {
        assert_eq!(
            Time::from_str("00:00:00").unwrap(),
            Time {
                hours: 0,
                minutes: 0,
                seconds: 0
            }
        );

        assert_eq!(
            Time::from_str("00:00:01").unwrap(),
            Time {
                hours: 0,
                minutes: 0,
                seconds: 1
            }
        );

        assert_eq!(
            Time::from_str("00:01:00").unwrap(),
            Time {
                hours: 0,
                minutes: 1,
                seconds: 0
            }
        );

        assert_eq!(
            Time::from_str("01:00:00").unwrap(),
            Time {
                hours: 1,
                minutes: 0,
                seconds: 0
            }
        );

        assert_eq!(
            Time::from_str("01:00:59").unwrap(),
            Time {
                hours: 1,
                minutes: 0,
                seconds: 59
            }
        );

        assert_eq!(
            Time::from_str("01:01:00").unwrap(),
            Time {
                hours: 1,
                minutes: 1,
                seconds: 0
            }
        );

        assert_eq!(
            Time::from_str("01:01:01").unwrap(),
            Time {
                hours: 1,
                minutes: 1,
                seconds: 1
            }
        );

        assert_eq!(
            Time::from_str("00:00:127").unwrap(),
            Time {
                hours: 0,
                minutes: 2,
                seconds: 7
            }
        );

        assert_eq!(
            Time::from_str("00:127:127").unwrap(),
            Time {
                hours: 2,
                minutes: 9,
                seconds: 7
            }
        );

        assert_eq!(
            Time::from_str("125:127:127").unwrap(),
            Time {
                hours: 127,
                minutes: 9,
                seconds: 7
            }
        );

        Time::from_str("127:127:127").unwrap_err();
    }
}
