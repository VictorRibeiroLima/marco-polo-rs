use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, PartialEq, Deserialize)]
pub enum AuthType {
    #[serde(rename = "OAUTH2")]
    Oauth2,
}

impl Display for AuthType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthType::Oauth2 => write!(f, "OAUTH2"),
        }
    }
}

impl FromStr for AuthType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "OAUTH2" => Ok(AuthType::Oauth2),
            _ => Err(format!("{} is not a valid AuthType", s)),
        }
    }
}
