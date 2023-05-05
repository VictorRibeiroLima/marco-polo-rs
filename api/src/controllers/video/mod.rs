use actix_web::{
    web::{self, post},
    HttpResponse, Responder,
};
use marco_polo_rs_core::{
    database::queries::{self, video::CreateVideoDto},
    internals::{
        cloud::{aws::s3::S3Client, traits::BucketClient},
        yt_downloader::{
            traits::{YoutubeDownloader, YoutubeVideoConfig},
            yt_dl::YtDl,
        },
    },
};

use crate::{middleware::jwt_token::TokenClaims, models::error::AppError, GlobalState};

use self::dtos::create::CreateVideo;

mod dtos;
mod state;

async fn create_video<YD, BC>(
    global_state: web::Data<GlobalState>,
    state: web::Data<state::State<YD, BC>>,
    jwt: TokenClaims,
    body: web::Json<CreateVideo>,
) -> Result<impl Responder, AppError>
where
    YD: YoutubeDownloader,
    BC: BucketClient,
{
    let pool = &global_state.pool;
    queries::channel::find_by_id(pool, body.channel_id).await?;

    let config = YoutubeVideoConfig {
        url: body.video_url.clone(),
        format: body.format.clone(),
        end_time: body.end_time.clone(),
        start_time: body.start_time.clone(),
    };

    let (file, video_id) = state.youtube_downloader.download(config).await?;

    let format: String = match &body.format {
        Some(format) => format.to_string(),
        None => "mkv".into(),
    };

    let language = match &body.language {
        Some(language) => language.to_string(),
        None => "en".into(),
    };

    let file_uri = format!("videos/raw/{}.{}", video_id, format);

    state.bucket_client.upload_file(&file_uri, file).await?;

    queries::video::create(
        pool,
        CreateVideoDto {
            id: &video_id,
            user_id: jwt.id,
            title: &body.title,
            description: &body.description,
            channel_id: body.channel_id,
            language: &language,
        },
    )
    .await?;

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
