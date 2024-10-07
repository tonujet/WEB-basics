use crate::api::auth::auth_service;
use crate::api::user::LoginUserDto;
use crate::api::utils::validation_extractor::JsonValidation;
use crate::api::{auth, AppState};
use poem::web::{Data, Json};
use poem::{handler, post, Route};

pub fn routes() -> Route {
    Route::new().at("/login", post(tokens))
}

#[handler]
async fn tokens(
    Data(state): Data<&AppState>,
    JsonValidation(dto): JsonValidation<LoginUserDto>,
) -> poem::Result<Json<auth::Tokens>> {
    let tokens = auth_service::login(state, dto).await?;
    Ok(Json(tokens))
}
