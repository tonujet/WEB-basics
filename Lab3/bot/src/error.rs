use thiserror::Error;


pub type AppResult<T> = Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Something went wrong with configuration. Details: {0}")]
    Config(String),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
    
    #[error("Something went wrong with the bot. Details: {0}")]
    Bot(String)
}
