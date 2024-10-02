use serde_json::json;
use thiserror::Error;
use tokio::sync::mpsc::error::SendError;
use warp::ws;

#[derive(Error, Debug)]
pub enum ChatError {
    #[error("User with name {0} already exist in this chat")]
    UsernameTaken(String),

    #[error("Failed to send a message")]
    Disconnect(#[from] SendError<ws::Message>),

    #[error(transparent)]
    InternalError(#[from] warp::Error),

    #[error("Invalid message")]
    InvalidMessage(),

    #[error("Body format is wrong")]
    InvalidMessageBody(#[from] serde_json::Error),
}

impl ChatError {
    pub fn to_request_body(&self) -> ws::Message {
        let message = match self {
            ChatError::UsernameTaken(_)
            | ChatError::Disconnect(_)
            | ChatError::InvalidMessage()
            | ChatError::InvalidMessageBody(_) => self.to_string(),
            _ => "Something went wrong".to_string(),
        };
        let body = serde_json::to_string(&json!({
            "error_message": message,
        }))
        .unwrap();
        ws::Message::text(body)
    }
}
