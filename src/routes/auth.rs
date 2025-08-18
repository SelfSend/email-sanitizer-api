use crate::auth::{User, generate_api_key};
use actix_web::{HttpResponse, Result, web};
use bcrypt::{DEFAULT_COST, hash};
use mongodb::{Client, Collection, bson::doc};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct ApiKeyResponse {
    pub api_key: String,
}

pub async fn register_and_generate_key(
    req: web::Json<RegisterRequest>,
    mongo_client: web::Data<Client>,
) -> Result<HttpResponse> {
    let db_name = env::var("DB_NAME_PRODUCTION").unwrap_or_else(|_| "email_sanitizer".to_string());
    let collection_name = env::var("DB_USERS_COLLECTION").unwrap_or_else(|_| "users".to_string());
    let db = mongo_client.database(&db_name);
    let collection: Collection<User> = db.collection(&collection_name);

    let password_hash = hash(&req.password, DEFAULT_COST)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Password hashing failed"))?;

    let user = User {
        email: req.email.clone(),
        password_hash: password_hash.clone(),
        active: true,
    };

    collection
        .insert_one(&user)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;

    let api_key = generate_api_key(&req.email, &password_hash)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Key generation failed"))?;

    Ok(HttpResponse::Ok().json(ApiKeyResponse { api_key }))
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/register", web::post().to(register_and_generate_key));
}
