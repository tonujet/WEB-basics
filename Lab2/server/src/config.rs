use std::env;
use std::sync::OnceLock;

use crate::error::{AppError, AppResult};

pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();
    INSTANCE.get_or_init(|| {
        Config::load().unwrap_or_else(|ex| panic!("ERROR WHILE LOADING CONF: {ex:?}"))
    })
}

trait ConfigLoader {
    fn load() -> AppResult<Self>
    where
        Self: Sized;
}

#[allow(non_snake_case)]
pub struct Config {
    pub SERVER: ServerConfig,
    pub JWT: JWTConfig,
}

impl ConfigLoader for Config {
    fn load() -> AppResult<Self>
    where
        Self: Sized,
    {
        Ok(Config {
            SERVER: ServerConfig::load()?,
            JWT: JWTConfig::load()?,
        })
    }
}
#[allow(non_snake_case)]
pub struct ServerConfig {
    pub PORT: u16,
    pub HOST: String,
}

impl ConfigLoader for ServerConfig {
    fn load() -> AppResult<Self>
    where
        Self: Sized,
    {
        let port: u16 = get_env("SERVER_PORT")?
            .parse()
            .map_err(|_| AppError::Config {
                env: "SERVER_PORT",
                message: "Can't not parse this variable".to_string(),
            })?;

        let host: String = get_env("SERVER_HOST")?;

        Ok(ServerConfig {
            PORT: port,
            HOST: host,
        })
    }
}

#[allow(non_snake_case)]
pub struct JWTConfig {
    pub SECRET: String,
    pub ACCESS_DURATION: u64,
}

impl ConfigLoader for JWTConfig {
    fn load() -> AppResult<Self>
    where
        Self: Sized,
    {
        let secret: String = get_env("JWT_SECRET")?;
        let access_duration: u64 =
            get_env("JWT_ACCESS_DURATION")?
                .parse()
                .map_err(|_| AppError::Config {
                    env: "JWT_ACCESS_DURATION",
                    message: "Can't not parse this variable".to_string(),
                })?;

        Ok(JWTConfig {
            SECRET: secret,
            ACCESS_DURATION: access_duration,
        })
    }
}

fn get_env(name: &'static str) -> AppResult<String> {
    env::var(name).map_err(|_| AppError::Config {
        env: name,
        message: "Can't read env variable from the file".to_string(),
    })
}
