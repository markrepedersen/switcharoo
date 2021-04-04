use actix_web::error::ErrorUnauthorized;
use actix_web::Error;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::lazy::SyncLazy;

static JWT_EXPIRATION_NUM_HOURS: i64 = 24;
static SECRET: SyncLazy<[u8; 32]> = SyncLazy::new(|| rand::thread_rng().gen::<[u8; 32]>());

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub username: String,
    pub permissions: Vec<String>,
    exp: i64,
}

impl Claims {
    pub fn new(username: String, permissions: Vec<String>) -> Self {
        Self {
            username,
            permissions,
            exp: (Utc::now() + Duration::hours(JWT_EXPIRATION_NUM_HOURS)).timestamp(),
        }
    }
}

/// Create a json web token (JWT)
pub fn create_jwt(claims: Claims) -> Result<String, Error> {
    let secret = SECRET.clone();
    let encoding_key = EncodingKey::from_secret(&secret);
    
    encode(&Header::default(), &claims, &encoding_key).map_err(|e| ErrorUnauthorized(e.to_string()))
}

/// Decode a json web token (JWT)
pub fn decode_jwt(token: &str) -> Result<Claims, Error> {
    let secret = SECRET.clone();
    let decoding_key = DecodingKey::from_secret(&secret);
    
    decode::<Claims>(token, &decoding_key, &Validation::default())
        .map(|data| data.claims)
        .map_err(|e| ErrorUnauthorized(e.to_string()))
}
