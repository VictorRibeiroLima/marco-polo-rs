use std::collections::HashSet;

use futures::future::join_all;
use marco_polo_rs_core::{
    database::{
        models::{channel::platform::Platform, user::UserRole},
        queries::{self, video::CreateVideoDto},
    },
    internals::{
        cloud::{
            models::payload::{PayloadType, VideoDownloadPayload},
            traits::QueueClient,
        },
        video_platform::youtube::traits::YoutubeClient as YoutubeClientTrait,
    },
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    middleware::jwt_token::TokenClaims,
    models::error::{AppError, AppErrorType},
};

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
        for channel_id in &cut.channel_ids {
            channel_ids.insert(*channel_id);
        }
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
) -> Result<(), Vec<AppError>> {
    let mut futures = vec![];
    for channel_id in channels {
        let future = check_channel_heath(pool, youtube_client, channel_id, &jwt);
        futures.push(future);
    }
    let results = join_all(futures).await;

    let errs = results
        .into_iter()
        .filter_map(|result| match result {
            Ok(_) => None,
            Err(err) => Some(err),
        })
        .collect::<Vec<AppError>>();

    if errs.is_empty() {
        return Ok(());
    }

    return Err(errs);
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

    match channel.platform {
        Platform::Youtube => {
            youtube_client.check_channel_health(&channel).await?;
        }
        _ => {
            return Err(AppError::new(
                AppErrorType::InternalServerError,
                "Not Implemented".into(),
            ))
        }
    }

    return Ok(());
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
        for i in 0..cut.channel_ids.len() {
            let dto = create_video_dto(cut, original_video_id, user_id, language, i).await;
            dtos.push(dto);
        }
    }
    return dtos;
}

//TODO: this is really ugly, refactor later
async fn create_video_dto<'a>(
    cut: &'a Cut,
    original_video_id: i32,
    user_id: i32,
    language: &'a str,
    i: usize,
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
        channel_id: cut.channel_ids[i],
        language: &language,
        original_id: original_video_id,
        tags,
        start_time,
    };

    return dto;
}
