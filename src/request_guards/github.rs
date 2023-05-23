use jwt_simple::prelude::*;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::State;

use crate::lib::env::Config;
use crate::models::auth::GithubWebhook;
use jwt_simple::prelude::*;

#[derive(Debug)]
pub enum AuthError {
    Missing,
    Invalid,
    Server,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for GithubWebhook {
    type Error = AuthError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Get the auth key from the rocket state.
        let config = match req.guard::<&State<Config>>().await {
            rocket::outcome::Outcome::Success(data) => data,
            _ => return Outcome::Failure((Status::InternalServerError, AuthError::Server)),
        };

        let key_bytes = hex::decode(&config.webhook_key).unwrap();
        let key = HS256Key::from_bytes(&key_bytes);

        let source_signature = req.headers().get("x-hub-signature-256");

        dbg!(req);

        return Outcome::Success(GithubWebhook { test: "a".to_string() });
    }
}
