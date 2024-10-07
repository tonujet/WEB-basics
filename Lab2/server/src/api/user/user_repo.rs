use crate::api::auth::hash_password;
use crate::api::error::{RepoError, RepoResult};
use crate::api::user::{CreateUserDto, UpdateUserDto, User, UserDto};
use crate::api::{AppState, Entity};

pub async fn list_users(state: &AppState) -> RepoResult<Vec<UserDto>> {
    let users = state.users.lock().await;
    println!("{users:?}");
    let users = users
        .clone()
        .into_iter()
        .map(|u| u.into())
        .collect::<Vec<_>>();
    Ok(users)
}

pub fn get_id(users: &[User]) -> RepoResult<u64> {
    let last_id = users
        .iter()
        .max_by_key(|u| u.id)
        .map(|u| u.to_owned())
        .unwrap_or_default()
        .id;
    Ok(if last_id == 0 { 0 } else { last_id + 1 })
}

pub async fn create_user(state: &AppState, create_user_dto: CreateUserDto) -> RepoResult<UserDto> {
    let mut users = state.users.lock().await;
    let CreateUserDto {
        username,
        password,
        desc,
        roles,
    } = create_user_dto;

    if users.iter().any(|u| u.username == username) {
        Err(RepoError::AlreadyExist(
            Entity::User,
            "username".to_string(),
        ))?;
    }
    if roles.is_empty() {
        Err(RepoError::Missing(Entity::User, "roles".to_string()))?;
    }

    let user = User {
        id: get_id(&users)?,
        username,
        password: hash_password(&password),
        desc,
        roles,
    };

    users.push(user.clone());
    Ok(user.into())
}

pub async fn get_user<P>(state: &AppState, mut predicate: P) -> RepoResult<UserDto>
where
    P: FnMut(UserDto) -> bool,
{
    let user = get_user_entity(state, |&u| predicate(u.clone().into())).await?;
    Ok(user.into())
}

async fn get_user_entity<P>(state: &AppState, predicate: P) -> RepoResult<User>
where
    P: FnMut(&&User) -> bool,
{
    let users = state.users.lock().await;
    Ok(users
        .iter()
        .find(predicate)
        .ok_or(RepoError::NotFound(Entity::User))?
        .to_owned())
}

pub async fn update_user(
    state: &AppState,
    UpdateUserDto { id, desc }: UpdateUserDto,
) -> RepoResult<UserDto> {
    let user_id = id.ok_or(RepoError::Internal)?;
    let user = get_user_entity(state, |u| u.id == user_id).await?;
    let updated_user = User {
        id: user.id,
        username: user.username,
        password: user.password,
        desc,
        roles: user.roles,
    };
    // users.retain(|u| u.id != updated_user.id);
    delete_user(&state, user_id).await?;
    state.users.lock().await.push(updated_user.clone());
    Ok(updated_user.into())
}

pub async fn delete_user(state: &AppState, id: u64) -> RepoResult<UserDto> {
    let user = get_user_entity(&state, |u| u.id == id).await?;
    state.users.lock().await.retain(|u| u.id != id);
    Ok(user.into())
}

pub async fn get_hashed_password(
    state: &AppState,
    username: &str,
) -> RepoResult<(UserDto, String)> {
    let user = get_user_entity(&state, |u| u.username == username).await?;
    let hashed_password = user.password.clone();
    Ok((user.into(), hashed_password))
}
