use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use chrono::{Duration, Utc};
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user_id
    pub exp: usize,  // expiration
    pub iat: usize,  // issued at
}

pub struct AuthService;

impl AuthService {
    pub fn create_token(user_id: &str) -> Result<String> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("valid timestamp")
            .timestamp() as usize;

        let issued_at = Utc::now().timestamp() as usize;

        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiration,
            iat: issued_at,
        };

        let secret = env::var("JWT_SECRET")?;
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )?;

        Ok(token)
    }

    pub fn verify_token(token: &str) -> Result<Claims> {
        let secret = env::var("JWT_SECRET")?;
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        )?;

        Ok(token_data.claims)
    }
} 