use serde::Serialize;
use sha3::{Digest, Sha3_512};

pub mod auth_controller;
pub mod auth_middleware;
mod auth_service;

#[derive(Debug, Serialize)]
pub struct Tokens {
    access_token: String,
    // refresh_token: String,
}

pub fn hash_password(password: &str) -> String {
    let password_hash = Sha3_512::digest(password);
    format!("{:x}", password_hash)
}
