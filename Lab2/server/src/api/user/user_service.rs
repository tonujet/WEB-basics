use crate::api::auth::hash_password;
use crate::api::error::{ApiResult, AuthError};
use crate::api::user::{user_repo, CreateUserDto, LoginUserDto, Role, UpdateUserDto, UserDto};
use crate::api::AppState;

pub async fn list_users(state: &AppState) -> ApiResult<Vec<UserDto>> {
    let users = user_repo::list_users(state).await?;
    Ok(users)
}

pub async fn create_user(state: &AppState, create_user_dto: CreateUserDto) -> ApiResult<UserDto> {
    let user = user_repo::create_user(state, create_user_dto).await?;
    Ok(user)
}

pub async fn validate_credentials(
    state: &AppState,
    LoginUserDto { username, password }: LoginUserDto,
) -> ApiResult<UserDto> {
    let (user_dto, stored_password) = user_repo::get_hashed_password(state, &username).await?;
    let password = hash_password(&password);

    if stored_password != password {
        return Err(AuthError::PasswordWrong(password).into());
    }

    Ok(user_dto)
}


pub async fn validate_roles(
    state: &AppState,
    username: &str,
    expected_roles: &[Role],
) -> ApiResult<UserDto> {
    let user = user_repo::get_user(state, |u| u.username == username).await?;

    let is_verified = expected_roles
        .iter()
        .any(|r1| user.roles.iter().any(|r2| r1 == r2));

    if !is_verified {
        Err(AuthError::InappropriateRole {
            user_roles: user.roles,
            needed_roles: expected_roles.to_owned(),
        })?
    } else {
        Ok(user)
    }
}


pub async fn update_user(state: &AppState, update_user_dto: UpdateUserDto) -> ApiResult<UserDto> {
    let user = user_repo::update_user(state, update_user_dto).await?;
    Ok(user)
}
