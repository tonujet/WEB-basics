use crate::api::error::{ApiError, ApiResult, AuthError};
use crate::api::jwt::jwt_service;
use crate::api::user::{user_service, Role};
use crate::api::AppState;
use crate::config::config;
use poem::http::HeaderMap;
use poem::{Endpoint, IntoResponse, Middleware, Request, Response};

pub struct AuthMiddleware {
    roles: Vec<Role>,
    state: AppState,
}

impl AuthMiddleware {
    pub fn admin(state: AppState) -> Self {
        Self {
            roles: vec![Role::Admin],
            state,
        }
    }
    pub fn user(state: AppState) -> Self {
        Self {
            roles: vec![Role::User],
            state,
        }
    }
    pub fn all(state: AppState) -> Self {
        Self {
            roles: vec![Role::Admin, Role::User],
            state,
        }
    }
}

impl<E: Endpoint> Middleware<E> for AuthMiddleware {
    type Output = AuthMiddlewareImpl<E>;

    fn transform(&self, ep: E) -> Self::Output {
        AuthMiddlewareImpl {
            ep,
            roles: self.roles.clone(),
            state: self.state.clone(),
        }
    }
}

pub struct AuthMiddlewareImpl<E> {
    ep: E,
    roles: Vec<Role>,
    state: AppState,
}

impl<E: Endpoint> Endpoint for AuthMiddlewareImpl<E> {
    type Output = Response;

    async fn call(&self, mut req: Request) -> poem::Result<Self::Output> {
        let token = extract_jwt_from_header(req.headers())?;
        let claims = jwt_service::validate_jwt(token, config().JWT.SECRET.as_ref())
            .map_err(ApiError::Authentication)?;
        let username = claims.iss;
        let user_dto = user_service::validate_roles(&self.state, &username, &self.roles).await?;
        req.extensions_mut().insert(user_dto);
        self.ep.call(req).await.map(|r| r.into_response())
    }
}

fn extract_jwt_from_header(headers: &HeaderMap) -> ApiResult<&str> {
    let res = headers
        .get("Authorization")
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_str| {
            if auth_str.starts_with("Bearer ") {
                Some(&auth_str[7..])
            } else {
                None
            }
        })
        .ok_or(AuthError::MissingToken.into());
    res
}
