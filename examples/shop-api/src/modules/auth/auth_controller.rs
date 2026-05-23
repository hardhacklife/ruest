use rustforge::prelude::*;

use super::dto::TokenResponse;

/// Connexion démo — émet un JWT (route publique via `SecurityConfig`).
#[controller("/auth")]
pub struct AuthController {
    jwt: Inject<JwtService>,
}

#[routes]
impl AuthController {
    /// `GET /auth/login` — token démo pour `demo@shop.local`.
    #[get("/login")]
    async fn login(&self) -> AppResult<Json<TokenResponse>> {
        let token = self
            .jwt
            .sign_subject(
                "demo@shop.local",
                vec!["user".into(), "admin".into()],
            )
            .map_err(|e| AppError::internal(e.to_string()))?;

        Ok(Json(TokenResponse {
            access_token: token,
            token_type: "Bearer",
            expires_in: SecurityConfig::dev().jwt_expires_in_secs,
        }))
    }
}
