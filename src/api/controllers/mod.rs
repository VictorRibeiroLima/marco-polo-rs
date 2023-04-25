use actix_web::web;

mod storage;

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.configure(storage::init_routes);
}
