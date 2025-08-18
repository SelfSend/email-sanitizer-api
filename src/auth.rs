use actix_web::dev::{Service, ServiceResponse, Transform, forward_ready};
use actix_web::error::ErrorUnauthorized;
use actix_web::{Error, Result, dev::ServiceRequest};
use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use mongodb::{Client, Collection, bson::doc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::future::{Ready, ready};
use std::pin::Pin;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub email: String,
    pub password_hash: String,
    pub active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub email: String,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiKey {
    pub key: String,
    pub active: bool,
}

pub struct AuthGuard;

pub fn generate_api_key(email: &str, password: &str) -> Result<String, Box<dyn std::error::Error>> {
    let jwt_secret = std::env::var("JWT_SECRET")?;
    let claims = Claims {
        email: email.to_string(),
        exp: (Utc::now() + Duration::days(30)).timestamp() as usize,
    };

    let mut hasher = Sha256::new();
    hasher.update(format!("{}{}", email, password));
    let input_hash = format!("{:x}", hasher.finalize());

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )?;
    Ok(format!("{}.{}", input_hash[..16].to_string(), token))
}

pub async fn verify_api_key(
    api_key: &str,
    mongo_client: &Client,
) -> Result<String, Box<dyn std::error::Error>> {
    let parts: Vec<&str> = api_key.splitn(2, '.').collect();
    if parts.len() != 2 {
        return Err("Invalid key format".into());
    }

    let jwt_secret = std::env::var("JWT_SECRET")?;
    let token_data = decode::<Claims>(
        parts[1],
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    )?;

    let db = mongo_client.database("email_sanitizer");
    let collection: Collection<User> = db.collection("users");

    if let Some(user) = collection
        .find_one(doc! { "email": &token_data.claims.email, "active": true })
        .await?
    {
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}", user.email, user.password_hash));
        let expected_prefix = format!("{:x}", hasher.finalize())[..16].to_string();

        if parts[0] == expected_prefix {
            return Ok(user.email);
        }
    }
    Err("Invalid API key".into())
}

pub struct AuthMiddleware<S> {
    service: S,
    mongo_client: Client,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let mongo_client = self.mongo_client.clone();
        let fut = self.service.call(req);

        Box::pin(async move {
            let (req, payload) = fut.await?.into_parts();

            let auth_header = req
                .headers()
                .get("Authorization")
                .and_then(|h| h.to_str().ok())
                .and_then(|s| s.strip_prefix("Bearer "));

            if let Some(api_key) = auth_header {
                match verify_api_key(api_key, &mongo_client).await {
                    Ok(_) => {
                        let res = ServiceResponse::new(req, payload);
                        Ok(res)
                    }
                    _ => Err(ErrorUnauthorized("Invalid API key")),
                }
            } else {
                Err(ErrorUnauthorized("Missing Authorization header"))
            }
        })
    }
}

pub struct Auth {
    mongo_client: Client,
}

impl Auth {
    pub fn new(mongo_client: Client) -> Self {
        Self { mongo_client }
    }
}

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware {
            service,
            mongo_client: self.mongo_client.clone(),
        }))
    }
}
