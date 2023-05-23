use crate::errors::response::CustomError;
use crate::models::auth::GithubWebhook;

use mongodb::bson::doc;
use mongodb::Database;
use rocket::State;

#[post("/webhook/<id>")]
pub async fn post(db: &State<Database>, _a: GithubWebhook, id: String) -> Result<(), CustomError> {
    todo!();
    Ok(())
}
