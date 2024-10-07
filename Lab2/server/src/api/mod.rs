use crate::api::auth::{auth_controller, hash_password};
use crate::api::user::{user_controller, Role, User};
use poem::middleware::{AddData, AddDataEndpoint};
use poem::{EndpointExt, Route};
use std::sync::Arc;
use strum_macros::AsRefStr;
use tokio::sync::Mutex;

mod auth;
mod error;
mod jwt;
mod user;
mod utils;

#[derive(Clone)]
pub struct AppState {
    users: Arc<Mutex<Vec<User>>>,
}

pub fn get_routes() -> AddDataEndpoint<Route, AppState> {
    let users = vec![
        User {
            id: 0,
            username: "user".to_string(),
            password: hash_password("user"),
            desc: Some("Some description".to_string()),
            roles: vec![Role::User],
        },
        User {
            id: 1,
            username: "admin".to_string(),
            password: hash_password("admin"),
            desc: None,
            roles: vec![Role::Admin],
        },
    ];

    let state = AppState {
        users: Arc::new(Mutex::new(users)),
    };
    let auth_routes = auth_controller::routes();
    let user_routes = user_controller::routes(state.clone());

    Route::new()
        .nest("/auth", auth_routes)
        .nest("/users", user_routes)
        .with(AddData::new(state))
}

#[derive(Debug, AsRefStr)]
enum Entity {
    User,
}
