use actix_web::web;

mod assembly_ai;
mod storage;
mod user;
mod video;

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.configure(storage::init_routes);
    config.configure(assembly_ai::init_routes);
    config.configure(user::init_routes);
    config.configure(video::init_routes);
}
