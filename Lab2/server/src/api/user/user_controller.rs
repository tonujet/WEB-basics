use crate::api::auth::auth_middleware::AuthMiddleware;
use crate::api::error::ApiError;
use crate::api::user::{user_service, CreateUserDto, UpdateUserDto, UserDto};
use crate::api::utils::validation_extractor::JsonValidation;
use crate::api::AppState;
use poem::web::{Data, Json};
use poem::{get, handler, EndpointExt, Request, Route};

pub fn routes(state: AppState) -> Route {
    Route::new()
        .at(
            "/",
            get(list_users)
                .post(add_user)
                .with(AuthMiddleware::admin(state.clone())),
        )
        .at(
            "/me",
            get(get_myself)
                .put(update_myself)
                .with(AuthMiddleware::all(state)),
        )
}

#[handler]
async fn list_users(Data(state): Data<&AppState>) -> poem::Result<Json<Vec<UserDto>>> {
    let users = user_service::list_users(state).await?;
    Ok(Json(users))
}

#[handler]
async fn add_user(
    Data(state): Data<&AppState>,
    JsonValidation(dto): JsonValidation<CreateUserDto>,
) -> poem::Result<Json<UserDto>> {
    let user = user_service::create_user(state, dto).await?;
    Ok(Json(user))
}

#[handler]
async fn get_myself(req: &Request) -> poem::Result<Json<UserDto>> {
    let user: &UserDto = req.extensions().get().ok_or(ApiError::Internal)?;
    Ok(Json(user.to_owned()))
}

#[handler]
async fn update_myself(
    req: &Request,
    Data(state): Data<&AppState>,
    JsonValidation(mut dto): JsonValidation<UpdateUserDto>,
) -> poem::Result<Json<UserDto>> {
    let user: &UserDto = req.extensions().get().unwrap();
    dto.id = Some(user.id);
    let updated_user = user_service::update_user(state, dto).await?;
    Ok(Json(updated_user))
}
