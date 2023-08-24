use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, PartialEq, Deserialize, sqlx::Type)]
#[sqlx(type_name = "video_platforms", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Platform {
    Youtube,
    Facebook,
    Instagram,
    Tiktok,
    Twitch,
    Vimeo,
    Dailymotion,
    Linkedin,
    Twitter,
    Pinterest,
    Snapchat,
    TikTok,
    Tumblr,
    Reddit,
    Whatsapp,
    Telegram,
    Vk,
    Ok,
    Weibo,
    Wechat,
    Line,
    KakaoTalk,
}

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Platform::Youtube => write!(f, "Youtube"),
            Platform::Facebook => write!(f, "Facebook"),
            Platform::Instagram => write!(f, "Instagram"),
            Platform::Tiktok => write!(f, "Tiktok"),
            Platform::Twitch => write!(f, "Twitch"),
            Platform::Vimeo => write!(f, "Vimeo"),
            Platform::Dailymotion => write!(f, "Dailymotion"),
            Platform::Linkedin => write!(f, "Linkedin"),
            Platform::Twitter => write!(f, "Twitter"),
            Platform::Pinterest => write!(f, "Pinterest"),
            Platform::Snapchat => write!(f, "Snapchat"),
            Platform::TikTok => write!(f, "TikTok"),
            Platform::Tumblr => write!(f, "Tumblr"),
            Platform::Reddit => write!(f, "Reddit"),
            Platform::Whatsapp => write!(f, "Whatsapp"),
            Platform::Telegram => write!(f, "Telegram"),
            Platform::Vk => write!(f, "Vk"),
            Platform::Ok => write!(f, "Ok"),
            Platform::Weibo => write!(f, "Weibo"),
            Platform::Wechat => write!(f, "Wechat"),
            Platform::Line => write!(f, "Line"),
            Platform::KakaoTalk => write!(f, "KakaoTalk"),
        }
    }
}

impl FromStr for Platform {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Youtube" => Ok(Platform::Youtube),
            "Facebook" => Ok(Platform::Facebook),
            "Instagram" => Ok(Platform::Instagram),
            "Tiktok" => Ok(Platform::Tiktok),
            "Twitch" => Ok(Platform::Twitch),
            "Vimeo" => Ok(Platform::Vimeo),
            "Dailymotion" => Ok(Platform::Dailymotion),
            "Linkedin" => Ok(Platform::Linkedin),
            "Twitter" => Ok(Platform::Twitter),
            "Pinterest" => Ok(Platform::Pinterest),
            "Snapchat" => Ok(Platform::Snapchat),
            "TikTok" => Ok(Platform::TikTok),
            "Tumblr" => Ok(Platform::Tumblr),
            "Reddit" => Ok(Platform::Reddit),
            "Whatsapp" => Ok(Platform::Whatsapp),
            "Telegram" => Ok(Platform::Telegram),
            "Vk" => Ok(Platform::Vk),
            "Ok" => Ok(Platform::Ok),
            "Weibo" => Ok(Platform::Weibo),
            "Wechat" => Ok(Platform::Wechat),
            "Line" => Ok(Platform::Line),
            "KakaoTalk" => Ok(Platform::KakaoTalk),
            _ => Err(format!(
                "{} is not a valid video platform. expected ('Youtube', 'Facebook', 'Instagram', 'Tiktok', 'Twitch', 'Vimeo', 'Dailymotion', 'Linkedin', 'Twitter', 'Pinterest', 'Snapchat', 'TikTok', 'Tumblr', 'Reddit', 'Whatsapp', 'Telegram', 'Vk', 'Ok', 'Weibo', 'Wechat', 'Line', 'KakaoTalk')",
                s
            )),
        }
    }
}
