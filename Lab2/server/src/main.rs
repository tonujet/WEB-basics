mod config;
mod error;
mod api;

use crate::config::config;
use crate::error::{AppError, AppResult};
use poem::{listener::TcpListener, Server};


// mongodb connection and proper repo level

// move logic from middleware to service
#[tokio::main]
async fn main() -> AppResult<()> {
    let socket_string = format!("{}:{}", config().SERVER.HOST, config().SERVER.PORT);
    let routes = api::get_routes();
    Server::new(TcpListener::bind(socket_string))
        .run(routes)
        .await
        .map_err(|error| AppError::Internal(error.to_string()))
}