use actix_web::{
    get,
    web::{self, Json},
    Responder, Scope,
};
use marco_polo_rs_core::{
    database::{
        models::{channel::Channel, user::UserRole},
        queries::{self, filter::Filter, pagination::Pagination},
    },
    internals::video_platform::youtube::{
        client::YoutubeClient, traits::YoutubeClient as YoutubeClientTrait,
    },
};

mod dto;
#[cfg(test)]
mod test;
mod youtube;

use crate::{
    controllers::channel::dto::ChannelDTO, middleware::jwt_token::TokenClaims,
    models::error::AppError, AppPool,
};

#[get("/{id}")]
async fn find_by_id(
    id: web::Path<i32>,
    pool: web::Data<AppPool>,
    _jwt: TokenClaims,
) -> Result<impl Responder, AppError> {
    let id = id.into_inner();
    let pool = &pool.pool;

    let channel = queries::channel::find_by_id(pool, id).await?;
    let dto: ChannelDTO = channel.into();

    return Ok(Json(dto));
}

#[get("")]
async fn find_all(
    pool: web::Data<AppPool>,
    pagination: web::Query<Pagination<Channel>>,
    filter: web::Query<Filter<Channel>>,
    jwt: TokenClaims,
) -> Result<impl Responder, AppError> {
    let pool = &pool.pool;
    let filter = filter.into_inner();
    let pagination = pagination.into_inner();

    let channels = match jwt.role {
        UserRole::Admin => queries::channel::find_all(pool, pagination, filter).await,
        UserRole::User => {
            let user_id = jwt.id;
            queries::channel::find_all_by_owner(pool, user_id, pagination, filter).await
        }
    }?;

    let dto: Vec<ChannelDTO> = channels.into_iter().map(|c| c.into()).collect();

    return Ok(Json(dto));
}

fn create_scope<YC: YoutubeClientTrait + 'static>() -> Scope {
    let youtube_scope = youtube::create_scope::<YC>();

    let channel_scope = web::scope("/channel")
        .service(find_by_id)
        .service(find_all)
        .service(youtube_scope);

    return channel_scope;
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    let channel_scope = create_scope::<YoutubeClient>();
    config.service(channel_scope);
}
