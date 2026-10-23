use std::future::{Ready, ready};
use std::rc::Rc;

use actix_web::{
    Error, HttpMessage,
    body::EitherBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
    error::ErrorUnauthorized,
    web,
};
use futures_util::future::LocalBoxFuture;

use crate::infrastructure::{config::env::Config, security::jwt::verify_access_token};

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();

        Box::pin(async move {
            let secret = match req.app_data::<web::Data<Config>>() {
                Some(cfg) => cfg.jwt_secret.clone(),
                None => {
                    return Err(ErrorUnauthorized("JWT secret not configured"));
                }
            };

            let auth_header = req
                .headers()
                .get("Authorization")
                .and_then(|v| v.to_str().ok());

            let token = match auth_header {
                Some(h) if h.starts_with("Bearer ") => &h[7..],
                _ => {
                    return Err(ErrorUnauthorized("Missing or invalid Authorization header"));
                }
            };

            match verify_access_token(&secret, token) {
                Ok(user_id) => {
                    req.extensions_mut().insert(user_id);
                    let res = svc.call(req).await?;
                    Ok(res.map_into_left_body())
                }
                Err(_) => Err(ErrorUnauthorized("Invalid or expired access token")),
            }
        })
    }
}
