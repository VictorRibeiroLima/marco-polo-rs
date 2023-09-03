use std::fmt::Display;

use crate::database::models::channel::Channel;

#[derive(Debug)]
pub enum HeathCheckError<'a> {
    ChannelNotFound(&'a Channel),
    ChannelNotConnected(&'a Channel),
    ChannelNotVerified(&'a Channel),
    ChannelNotMonetized(&'a Channel),
    ChannelNotEnabled(&'a Channel),
    ChannelNotEligible(&'a Channel),
    ChannelNotSupported(&'a Channel),
    ChannelNotReady(&'a Channel),
    ChannelNotActive(&'a Channel),
    ChannelNotAccessible(&'a Channel),
    ChannelHasDbError(&'a Channel),
    ChannelWrongAuthType(&'a Channel),
}

impl Display for HeathCheckError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HeathCheckError::ChannelNotFound(channel) => {
                write!(
                    f,
                    "Channel with id {} not found for platform: {}",
                    channel.id, channel.platform
                )
            }
            HeathCheckError::ChannelNotConnected(channel) => {
                write!(
                    f,
                    "Channel with id {} not connected for platform: {}",
                    channel.id, channel.platform
                )
            }
            HeathCheckError::ChannelNotVerified(channel) => {
                write!(
                    f,
                    "Channel with id {} not verified for platform: {}",
                    channel.id, channel.platform
                )
            }
            HeathCheckError::ChannelNotMonetized(channel) => {
                write!(
                    f,
                    "Channel with id {} not monetized for platform: {}",
                    channel.id, channel.platform
                )
            }
            HeathCheckError::ChannelNotEnabled(channel) => {
                write!(
                    f,
                    "Channel with id {} not enabled for platform: {}",
                    channel.id, channel.platform
                )
            }
            HeathCheckError::ChannelNotEligible(channel) => {
                write!(
                    f,
                    "Channel with id {} not eligible for platform: {}",
                    channel.id, channel.platform
                )
            }
            HeathCheckError::ChannelNotSupported(channel) => {
                write!(
                    f,
                    "Channel with id {} not supported for platform: {}",
                    channel.id, channel.platform
                )
            }
            HeathCheckError::ChannelNotReady(channel) => {
                write!(
                    f,
                    "Channel with id {} not ready for platform: {}",
                    channel.id, channel.platform
                )
            }
            HeathCheckError::ChannelNotActive(channel) => {
                write!(
                    f,
                    "Channel with id {} not active for platform: {}",
                    channel.id, channel.platform
                )
            }
            HeathCheckError::ChannelNotAccessible(channel) => {
                write!(
                    f,
                    "Channel with id {} not accessible for platform: {}",
                    channel.id, channel.platform
                )
            }
            HeathCheckError::ChannelHasDbError(channel) => {
                write!(
                    f,
                    "Channel with id {} has db error for platform: {}",
                    channel.id, channel.platform
                )
            }
            HeathCheckError::ChannelWrongAuthType(channel) => {
                write!(
                    f,
                    "Channel with id {} has wrong auth type for platform: {} and auth type: {}",
                    channel.id, channel.platform, channel.auth.0
                )
            }
        }
    }
}

impl std::error::Error for HeathCheckError<'_> {}
