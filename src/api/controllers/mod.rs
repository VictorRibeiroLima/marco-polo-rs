use actix_web::web;

mod assembly_ai;
mod middleware;
mod storage;

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.configure(storage::init_routes);
    config.configure(assembly_ai::init_routes);
}
