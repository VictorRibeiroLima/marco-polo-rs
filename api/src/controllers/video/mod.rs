use actix_web::{
    web::{self, post, Json},
    HttpResponse, Responder,
};
use marco_polo_rs_core::{
    database::queries,
    internals::{
        cloud::{aws::s3::S3Client, traits::BucketClient},
        yt_downloader::{
            traits::{YoutubeDownloader, YoutubeVideoConfig},
            yt_dl::YtDl,
        },
    },
};
use validator::Validate;

use crate::{middleware::jwt_token::TokenClaims, models::error::AppError, AppPool};

use self::dtos::create::CreateVideo;

mod dtos;
mod service;
mod state;

async fn create_video<YD, BC>(
    pool: web::Data<AppPool>,
    state: web::Data<state::State<YD, BC>>,
    jwt: TokenClaims,
    body: Json<CreateVideo>,
) -> Result<impl Responder, AppError>
where
    YD: YoutubeDownloader,
    BC: BucketClient,
{
    body.validate()?;
    let pool = &pool.pool;
    let body = body.into_inner();
    queries::channel::find_by_id(pool, body.channel_id).await?;

    let config = YoutubeVideoConfig {
        url: &body.video_url,
        format: &body.format,
        end_time: &body.end_time,
        start_time: &body.start_time,
    };

    let (file, video_id) = state.youtube_downloader.download(config).await?;

    service::create_video(pool, body, &state.bucket_client, jwt.id, video_id, file).await?;

    return Ok(HttpResponse::Created().finish());
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    let bucket_client = S3Client::new().unwrap();
    let youtube_downloader = YtDl;
    let state = state::State {
        bucket_client,
        youtube_downloader,
    };
    let state = web::Data::new(state);
    let scope = web::scope("/video");
    let scope = scope.app_data(state);
    let scope = scope.route("/", post().to(create_video::<YtDl, S3Client>));
    config.service(scope);
}
