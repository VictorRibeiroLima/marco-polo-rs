use std::collections::HashSet;

use futures::future::join_all;
use marco_polo_rs_core::{
    database::{
        models::{channel::auth::AuthType, user::UserRole},
        queries::{self, video::CreateVideoDto},
    },
    internals::{
        cloud::{
            models::payload::{PayloadType, VideoDownloadPayload},
            traits::QueueClient,
        },
        youtube_client::traits::YoutubeClient as YoutubeClientTrait,
    },
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{middleware::jwt_token::TokenClaims, models::error::AppError};

use super::dtos::create::{Create, Cut};

pub async fn create_video<QC: QueueClient, YC: YoutubeClientTrait>(
    pool: &PgPool,
    body: Create,
    queue_client: &QC,
    youtube_client: &YC,
    jwt: TokenClaims,
) -> Result<Vec<Uuid>, AppError> {
    let mut channel_ids = HashSet::new();
    let user_id = jwt.id;

    for cut in &body.cuts {
        channel_ids.insert(cut.channel_id);
    }

    check_channels_heath(pool, youtube_client, channel_ids, jwt).await?;
    let ids = create_videos(pool, body, user_id, queue_client).await?;
    return Ok(ids);
}

async fn check_channels_heath(
    pool: &PgPool,
    youtube_client: &impl YoutubeClientTrait,
    channels: HashSet<i32>,
    jwt: TokenClaims,
) -> Result<(), AppError> {
    let mut futures = vec![];
    for channel_id in channels {
        let future = check_channel_heath(pool, youtube_client, channel_id, &jwt);
        futures.push(future);
    }
    let results = join_all(futures).await;

    for result in results {
        match result {
            Ok(_) => {}
            Err(err) => return Err(err),
        }
    }
    return Ok(());
}

async fn check_channel_heath(
    pool: &PgPool,
    youtube_client: &impl YoutubeClientTrait,
    channel_id: i32,
    jwt: &TokenClaims,
) -> Result<(), AppError> {
    let channel = match jwt.role {
        UserRole::Admin => queries::channel::find_by_id(pool, channel_id).await?,
        UserRole::User => {
            let user_id = jwt.id;
            queries::channel::find_by_and_creator(pool, channel_id, user_id).await?
        }
    };

    if channel.error {
        return Err(AppError::bad_request(
            "Channel has errors. Please contact admins".to_string(),
        ));
    };

    //TODO: make generic
    let auth_type = match channel.auth.0 {
        AuthType::Oauth2(auth) => auth,
        _ => {
            return Err(AppError::bad_request(
                "Youtube channel not linked".to_string(),
            ))
        }
    };

    let refresh_token = match auth_type.refresh_token {
        Some(refresh_token) => refresh_token,
        None => {
            return Err(AppError::bad_request(
                "Youtube channel not linked".to_string(),
            ))
        }
    };

    let result = youtube_client.get_channel_info(refresh_token).await;

    if result.is_err() {
        queries::channel::change_error_state(pool, channel_id, true).await?;
        return Err(AppError::bad_request(
            "Channel has errors. Please contact admins".to_string(),
        ));
    }

    Ok(())
}

async fn create_videos(
    pool: &PgPool,
    body: Create,
    user_id: i32,
    queue_client: &impl QueueClient,
) -> Result<Vec<Uuid>, AppError> {
    let language = match &body.language {
        Some(language) => language,
        None => "en",
    };
    let mut trx = pool.begin().await?;
    let original_video_id = queries::original_video::create(&mut *trx, &body.video_url).await?;

    let dtos = create_video_dtos(&body, original_video_id, user_id, &language).await;
    let video_ids: Vec<Uuid> = dtos.iter().map(|dto| dto.id).collect();

    queries::video::create_many(&mut *trx, dtos).await?;

    let payload: VideoDownloadPayload = VideoDownloadPayload {
        original_video_id,
        video_ids: video_ids.clone(),
    };

    queue_client
        .send_message(PayloadType::BatukaDownloadVideo(payload))
        .await?;

    trx.commit().await?;

    return Ok(video_ids);
}

async fn create_video_dtos<'a>(
    body: &'a Create,
    original_video_id: i32,
    user_id: i32,
    language: &'a str,
) -> Vec<CreateVideoDto<'a>> {
    let mut dtos = vec![];
    for cut in &body.cuts {
        let dto = create_video_dto(cut, original_video_id, user_id, language).await;
        dtos.push(dto);
    }
    return dtos;
}

async fn create_video_dto<'a>(
    cut: &'a Cut,
    original_video_id: i32,
    user_id: i32,
    language: &'a str,
) -> CreateVideoDto<'a> {
    let video_id = uuid::Uuid::new_v4();

    let start_time = match &cut.start_time {
        Some(start_time) => start_time,
        None => "00:00:00",
    };

    let end_time: Option<&str> = match &cut.end_time {
        Some(end_time) => Some(end_time),
        None => None,
    };

    let tags: Option<String> = match &cut.tags {
        Some(tags) => {
            let tags = tags.join(";");
            Some(tags)
        }
        None => None,
    };

    let dto = CreateVideoDto {
        id: video_id,
        user_id,
        title: &cut.title,
        end_time,
        description: &cut.description,
        channel_id: cut.channel_id,
        language: &language,
        original_id: original_video_id,
        tags,
        start_time,
    };

    return dto;
}
