use crate::api::auth::Tokens;
use crate::api::error::ApiResult;
use crate::api::jwt::jwt_service;
use crate::api::user::{user_service, LoginUserDto};
use crate::api::AppState;
use crate::config::config;
use chrono::Duration;

pub async fn login(state: &AppState, user_dto: LoginUserDto) -> ApiResult<Tokens> {
    let user = user_service::validate_credentials(state, user_dto).await?;
    let subject = user
        .roles
        .iter()
        .map(|r| r.as_ref())
        .collect::<Vec<_>>()
        .join(",");

    let duration = Duration::seconds(config().JWT.ACCESS_DURATION as i64);
    let secret = config().JWT.SECRET.as_ref();

    let access_token = jwt_service::make_jwt(user.username, subject, duration, secret)?;

    Ok(Tokens { access_token })
}
