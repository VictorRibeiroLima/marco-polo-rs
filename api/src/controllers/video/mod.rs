use actix_web::web;

mod dtos;

pub fn init_routes(config: &mut web::ServiceConfig) {
    let scope = web::scope("/video");
    config.service(scope);
}
