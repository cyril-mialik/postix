use actix_web::{
    Error, FromRequest, HttpMessage, HttpRequest, dev::Payload, error::ErrorUnauthorized,
};
use futures_util::future::{Ready, ready};
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct AuthenticatedUser {
    pub user_id: Uuid,
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let user_id = req.extensions().get::<Uuid>().copied();

        match user_id {
            Some(user_id) => ready(Ok(AuthenticatedUser { user_id })),
            None => ready(Err(ErrorUnauthorized(
                "Authentication middleware not applied or token missing",
            ))),
        }
    }
}
