use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use actix_web::{
    Error, HttpMessage, HttpResponse,
    body::{EitherBody, MessageBody},
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    http::header,
};
use futures_util::future::{Ready, ok};

use crate::infrastructure::security::jwt;

#[derive(Clone)]
pub struct JwtAuth {
    secret: Arc<String>,
}

impl JwtAuth {
    pub fn new(secret: impl Into<String>) -> Self {
        Self {
            secret: Arc::new(secret.into()),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for JwtAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtAuthMiddleware {
            service,
            secret: self.secret.clone(),
        })
    }
}

pub struct JwtAuthMiddleware<S> {
    service: S,
    secret: Arc<String>,
}

impl<S, B> Service<ServiceRequest> for JwtAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let token = req
            .headers()
            .get(header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "))
            .map(str::to_owned);

        let Some(token) = token else {
            let (req, _) = req.into_parts();
            let res = HttpResponse::Unauthorized()
                .json(serde_json::json!({ "error": "missing bearer token" }));
            return Box::pin(
                async move { Ok(ServiceResponse::new(req, res).map_into_right_body()) },
            );
        };

        let user_id = match jwt::verify_access_token(&self.secret, &token) {
            Ok(id) => id,
            Err(_) => {
                let (req, _) = req.into_parts();
                let res = HttpResponse::Unauthorized()
                    .json(serde_json::json!({ "error": "invalid token" }));
                return Box::pin(async move {
                    Ok(ServiceResponse::new(req, res).map_into_right_body())
                });
            }
        };

        req.extensions_mut().insert(user_id);

        let fut = self.service.call(req);
        Box::pin(async move { fut.await.map(|res| res.map_into_left_body()) })
    }
}
