use crate::controllers;
use actix_web::{
    self,
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
    middleware::NormalizePath,
    App, Error,
};

pub mod mock;

pub fn create_test_app() -> App<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse,
        Error = Error,
        InitError = (),
    >,
> {
    let app = App::new()
        .wrap(NormalizePath::trim())
        .configure(controllers::init_routes);
    return app;
}
