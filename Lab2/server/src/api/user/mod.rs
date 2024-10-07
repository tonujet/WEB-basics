pub mod user_controller;
pub mod user_repo;
pub mod user_service;

use serde::{Deserialize, Serialize};
use strum_macros::AsRefStr;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub password: String,
    pub desc: Option<String>,
    pub roles: Vec<Role>,
}

impl From<UserDto> for User {
    fn from(
        UserDto {
            id,
            username,
            desc,
            roles,
        }: UserDto,
    ) -> Self {
        User {
            id,
            username,
            password: Default::default(),
            desc,
            roles,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct UserDto {
    pub id: u64,
    pub username: String,
    pub desc: Option<String>,
    pub roles: Vec<Role>,
}

impl From<User> for UserDto {
    fn from(
        User {
            id,
            username,
            password: _password,
            desc,
            roles,
        }: User,
    ) -> Self {
        Self {
            id,
            username,
            desc,
            roles,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct LoginUserDto {
    #[validate(length(min = 3, max = 100, message = "Must be between 3 and 30 characters"))]
    pub username: String,

    #[validate(length(min = 3, max = 100, message = "Must be between 3 and 1000 characters"))]
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateUserDto {
    #[validate(length(min = 3, max = 100, message = "Must be between 3 and 30 characters"))]
    pub username: String,

    #[validate(length(min = 3, max = 100, message = "Must be between 3 and 1000 characters"))]
    pub password: String,

    #[validate(length(min = 3, max = 1000, message = "Must be between 3 and 1000 characters"))]
    pub desc: Option<String>,

    #[validate(length(min = 1, message = "Must be at least one role"))]
    pub roles: Vec<Role>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UpdateUserDto {
    pub id: Option<u64>,

    #[validate(length(min = 3, max = 1000, message = "Must be between 3 and 1000 characters"))]
    pub desc: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, AsRefStr)]
pub enum Role {
    User,
    Admin,
}
