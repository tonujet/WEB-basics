use crate::api::error::AuthResult;
use chrono::{Duration, Local};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub iss: String,
    pub sub: String,
    pub iat: i64,
    pub exp: i64,
}

pub fn make_jwt(
    issuer: String,
    subject: String,
    duration: Duration,
    secret: &[u8],
) -> AuthResult<String> {
    let mut header = Header::new(Algorithm::HS256);
    header.typ = Some("JWT".to_string());

    let now = Local::now();
    let iat = now.timestamp();
    let exp = (now + duration).timestamp();
    let claims = Claims {
        iss: issuer,
        sub: subject,
        iat,
        exp,
    };

    Ok(encode(&header, &claims, &EncodingKey::from_secret(secret))?)
}

pub fn validate_jwt(token: &str, secret: &[u8]) -> AuthResult<Claims> {
    let validation = Validation::default();
    let token = decode::<Claims>(token, &DecodingKey::from_secret(secret), &validation)?;
    Ok(token.claims)
}
