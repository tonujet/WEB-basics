use crate::error::{AppError, AppResult};
use std::env;
use std::sync::OnceLock;

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
    pub OPENAI: OpenaiConfig,
    pub GROQ: GroqConfig,
    pub TELEGRAM: TelegramConfig,
}

#[allow(non_snake_case)]
pub struct OpenaiConfig {
    pub API_KEY: String,
}

#[allow(non_snake_case)]
pub struct GroqConfig {
    pub API_KEY: String,
}

#[allow(non_snake_case)]
pub struct TelegramConfig {
    pub TOKEN: String,
}

impl ConfigLoader for Config {
    fn load() -> AppResult<Self>
    where
        Self: Sized
    {
        Ok(Self {
            OPENAI: OpenaiConfig::load()?,
            GROQ: GroqConfig::load()?,
            TELEGRAM: TelegramConfig::load()?,
        })
    }
}

impl ConfigLoader for OpenaiConfig {
    fn load() -> AppResult<Self>
    where
        Self: Sized,
    {
        let api_key = get_env("OPENAI_API_KEY")?;
        Ok(Self { API_KEY: api_key })
    }
}

impl ConfigLoader for GroqConfig {
    fn load() -> AppResult<Self>
    where
        Self: Sized,
    {
        let api_key = get_env("GROQ_API_KEY")?;
        Ok(Self { API_KEY: api_key })
    }
}

impl ConfigLoader for TelegramConfig {
    fn load() -> AppResult<Self>
    where
        Self: Sized,
    {
        let token = get_env("TELEGRAM_TOKEN")?;
        Ok(Self { TOKEN: token })
    }
}

fn get_env(name: &'static str) -> AppResult<String> {
    Ok(env::var(name).map_err(|_| AppError::Config(format!("Can't read env variable {name}")))?)
}
