use crate::api::user::Role;
use crate::api::Entity;
use jsonwebtoken::errors::ErrorKind;
use poem::error::ResponseError;
use poem::http::StatusCode;
use poem::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use std::error::Error as StdError;
use strum_macros::AsRefStr;
use thiserror::Error;

pub type ApiResult<T> = Result<T, ApiError>;
pub type AuthResult<T> = Result<T, AuthError>;
pub type RepoResult<T> = Result<T, RepoError>;

#[derive(Error, Debug, AsRefStr)]
pub enum ApiError {
    #[error(transparent)]
    Authentication(#[from] AuthError),

    #[error(transparent)]
    Repository(#[from] RepoError),

    #[error(transparent)]
    Validation(#[from] validator::ValidationErrors),

    #[error(transparent)]
    Parsing(#[from] poem::error::ParseJsonError),

    #[error("Something went wrong")]
    Internal,
}

impl ResponseError for ApiError {
    fn status(&self) -> StatusCode {
        match self {
            ApiError::Authentication(err) => err.status(),
            ApiError::Repository(err) => err.status(),
            ApiError::Parsing(err) => err.status(),
            ApiError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Validation(_) => StatusCode::CONFLICT,
        }
    }

    fn as_response(&self) -> Response
    where
        Self: StdError + Send + Sync + 'static,
    {
        let code = self.status();
        let body = ResponseBody {
            status_code: code.as_str(),
            status_code_message: code.canonical_reason().unwrap_or("Unknown"),
            message: self.to_string(),
            error_name: format!("{}Error", self.as_ref()),
        };
        let body = serde_json::to_string(&body)
            .unwrap_or("Something went wrong with exception handling".to_string());

        body.with_content_type("application/json")
            .with_status(self.status())
            .into_response()
    }
}

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Password {0} wrong to this user")]
    PasswordWrong(String),

    #[error(
        "This part only accessible for this roles: {needed_roles:?}, but you have: {user_roles:?}"
    )]
    InappropriateRole {
        user_roles: Vec<Role>,
        needed_roles: Vec<Role>,
    },

    #[error("There isn't token in the header. Try to add Authorization: Bearer xxx")]
    MissingToken,

    #[error(transparent)]
    JWThandling(#[from] jsonwebtoken::errors::Error),
}

impl ResponseError for AuthError {
    fn status(&self) -> StatusCode {
        match self {
            AuthError::PasswordWrong(_) => StatusCode::UNAUTHORIZED,
            AuthError::InappropriateRole { .. } => StatusCode::FORBIDDEN,
            AuthError::MissingToken => StatusCode::UNAUTHORIZED,
            AuthError::JWThandling(err) => match err.kind() {
                ErrorKind::InvalidToken
                | ErrorKind::InvalidSignature
                | ErrorKind::ExpiredSignature => StatusCode::UNAUTHORIZED,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
        }
    }
}

#[derive(Error, Debug)]
pub enum RepoError {
    #[error("Entity {} not found", .0.as_ref())]
    NotFound(Entity),

    #[error("Entity {} with field: {1} already exist", .0.as_ref())]
    AlreadyExist(Entity, String),

    #[error("Entity {} is missing smth in field: {1} ", .0.as_ref())]
    Missing(Entity, String),

    #[error("Something went wrong")]
    Internal,
}

impl ResponseError for RepoError {
    fn status(&self) -> StatusCode {
        match self {
            RepoError::NotFound(_) => StatusCode::NOT_FOUND,
            RepoError::AlreadyExist(_, _) => StatusCode::CONFLICT,
            RepoError::Missing(_, _) => StatusCode::UNPROCESSABLE_ENTITY,
            RepoError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ResponseBody<'a> {
    status_code: &'a str,
    status_code_message: &'a str,
    message: String,
    error_name: String,
}
