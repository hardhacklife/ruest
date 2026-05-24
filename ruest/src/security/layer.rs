use std::sync::Arc;

use axum::body::Body;
use axum::extract::State;
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

use super::config::SecurityConfig;
use super::context::AuthContext;
use super::jwt::JwtService;
use super::SecurityError;

/// État partagé du middleware JWT.
#[derive(Clone)]
pub struct JwtAuthState {
    pub jwt: Arc<JwtService>,
    pub public_routes: Arc<Vec<String>>,
}

impl JwtAuthState {
    pub fn new(jwt: Arc<JwtService>, config: &SecurityConfig) -> Self {
        Self {
            jwt,
            public_routes: Arc::new(config.public_routes.clone()),
        }
    }

    fn is_public(&self, path: &str) -> bool {
        self.public_routes
            .iter()
            .any(|p| path == p.as_str() || path.starts_with(&format!("{p}/")))
    }
}

/// Middleware Axum : routes publiques sans token, sinon Bearer JWT obligatoire.
pub async fn jwt_auth_middleware(
    State(state): State<JwtAuthState>,
    mut req: Request<Body>,
    next: Next,
) -> Response {
    let path = req.uri().path().to_string();

    if state.is_public(&path) {
        return next.run(req).await;
    }

    match extract_bearer(req.headers()) {
        Ok(token) => match state.jwt.verify(&token) {
            Ok(claims) => {
                req.extensions_mut().insert(AuthContext {
                    claims,
                    token,
                });
                next.run(req).await
            }
            Err(e) => unauthorized_json(e),
        },
        Err(e) => unauthorized_json(e),
    }
}

fn extract_bearer(
    headers: &axum::http::HeaderMap,
) -> Result<String, SecurityError> {
    let value = headers
        .get(axum::http::header::AUTHORIZATION)
        .ok_or(SecurityError::MissingAuthorization)?
        .to_str()
        .map_err(|_| SecurityError::InvalidScheme)?;

    let token = value
        .strip_prefix("Bearer ")
        .or_else(|| value.strip_prefix("bearer "))
        .ok_or(SecurityError::InvalidScheme)?;

    if token.is_empty() {
        return Err(SecurityError::InvalidToken);
    }

    Ok(token.to_string())
}

fn unauthorized_json(err: SecurityError) -> Response {
    let status = StatusCode::UNAUTHORIZED;
    let body = json!({
        "status": status.as_u16(),
        "message": err.to_string(),
    });
    (status, Json(body)).into_response()
}

/// Applique le middleware JWT sur un routeur Axum.
pub fn apply_jwt_layer(
    router: axum::Router,
    jwt: Arc<JwtService>,
    config: &SecurityConfig,
) -> axum::Router {
    let state = JwtAuthState::new(jwt, config);
    router.layer(axum::middleware::from_fn_with_state(state, jwt_auth_middleware))
}
