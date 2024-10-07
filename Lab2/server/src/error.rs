use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Problem with {env:?} env variable. Additional info {message:?}")]
    Config { env: &'static str, message: String },

    #[error("Internal error: {0}")]
    Internal (String)
}


