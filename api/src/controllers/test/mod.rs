use actix_web::{
    self,
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
    middleware::NormalizePath,
    web::{JsonConfig, QueryConfig},
    App, Error,
};

use crate::models::error::AppError;

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
        .app_data(JsonConfig::default().error_handler(|err, _req| {
            let error = AppError::from(err);
            return error.into();
        }))
        .app_data(QueryConfig::default().error_handler(|err, _req| {
            let error = AppError::from(err);
            return error.into();
        }));
    return app;
}
