macro_rules! authorization {
    ($name:ident,$key_name: expr) => {
        use std::future::{ready, Ready};

        use actix_web::{
            dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
            Error,
        };
        use futures::future::LocalBoxFuture;

        struct $name;

        impl<S, B> Transform<S, ServiceRequest> for $name
        where
            S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
            S::Future: 'static,
            B: 'static,
        {
            type Response = ServiceResponse<B>;
            type Error = Error;
            type InitError = ();
            type Transform = Transformer<S>;
            type Future = Ready<Result<Self::Transform, Self::InitError>>;

            fn new_transform(&self, service: S) -> Self::Future {
                ready(Ok(Transformer { service }))
            }
        }

        struct Transformer<S> {
            service: S,
        }

        impl<S, B> Service<ServiceRequest> for Transformer<S>
        where
            S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
            S::Future: 'static,
            B: 'static,
        {
            type Response = ServiceResponse<B>;
            type Error = Error;
            type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

            forward_ready!(service);

            fn call(&self, req: ServiceRequest) -> Self::Future {
                let auth_header = req.headers().get("Authorization");
                let key = std::env::var($key_name).unwrap();

                let auth_header = match auth_header {
                    Some(header) => header,
                    None => {
                        return Box::pin(async move {
                            Err(
                                AppError::unauthorized("Missing Authorization header".to_string())
                                    .into(),
                            )
                        })
                    }
                };

                let header_value = auth_header.to_str().unwrap_or("").to_string();
                if header_value != key {
                    return Box::pin(async move {
                        Err(AppError::unauthorized("Invalid API key".to_string()).into())
                    });
                }

                let fut = self.service.call(req);

                Box::pin(async move {
                    let res = fut.await?;

                    Ok(res)
                })
            }
        }
    };
}

pub(crate) use authorization;
