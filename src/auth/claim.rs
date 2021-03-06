use actix_web::error::ErrorUnauthorized;
use actix_web::Error;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::lazy::SyncLazy;
use uuid::Uuid;

pub static JWT_DEFAULT_EXPIRATION: i64 = 24;

static SECRET: SyncLazy<[u8; 32]> = SyncLazy::new(|| thread_rng().gen::<[u8; 32]>());

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub username: String,
    pub tenant_id: Uuid,
    pub permissions: Vec<String>,
    pub exp: i64,
}

impl Claims {
    /// Create a new Claim that can be retrieved from decoding a JWT.
    /// # username
    /// The username of the user
    /// # exp
    /// The expiry time for the JWT in seconds
    /// # tenant_id
    /// The user's tenant id
    /// # permissions
    /// The user's permissions
    pub fn new(username: String, exp: i64, tenant_id: Uuid, permissions: Vec<String>) -> Self {
        Self {
            username,
            tenant_id,
            permissions,
            exp: (Utc::now() + Duration::hours(exp)).timestamp(),
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
