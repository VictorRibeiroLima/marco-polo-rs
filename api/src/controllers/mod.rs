use actix_web::web;

mod assembly_ai;
mod channel;
mod storage;
mod user;
mod video;

#[cfg(test)]
mod test;

pub fn init_routes(config: &mut web::ServiceConfig) {
    println!("Initializing routes...");
    config.configure(storage::init_routes);
    config.configure(assembly_ai::init_routes);
    config.configure(user::init_routes);
    config.configure(video::init_routes);
    config.configure(channel::init_routes);
}
