use std::sync::Arc;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::routing::get;
use axum::Router;
use rustforge_security::{
    apply_jwt_layer, AuthUser, JwtService, SecurityConfig,
};
use tower::ServiceExt;

#[test]
fn sign_and_verify_roundtrip() {
    let config = SecurityConfig::dev();
    let jwt = JwtService::new(&config).expect("jwt");

    let token = jwt
        .sign_subject("user-1", vec!["user".into()])
        .expect("sign");

    let claims = jwt.verify(&token).expect("verify");
    assert_eq!(claims.sub, "user-1");
    assert!(claims.has_role("user"));
}

#[tokio::test]
async fn middleware_blocks_without_token() {
    let config = SecurityConfig::dev();
    let jwt = Arc::new(JwtService::new(&config).expect("jwt"));

    let app = apply_jwt_layer(
        Router::new().route("/private", get(|| async { "secret" })),
        jwt,
        &config,
    );

    let response = app
        .oneshot(
            Request::builder()
                .uri("/private")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn middleware_allows_public_and_bearer() {
    let config = SecurityConfig::dev();
    let jwt = Arc::new(JwtService::new(&config).expect("jwt"));
    let token = jwt.sign_subject("u1", vec![]).expect("token");

    async fn me(user: AuthUser) -> String {
        user.subject().to_string()
    }

    let app = apply_jwt_layer(
        Router::new()
            .route("/health", get(|| async { "ok" }))
            .route("/me", get(me)),
        jwt,
        &config,
    );

    let health = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(health.status(), StatusCode::OK);

    let me = app
        .oneshot(
            Request::builder()
                .uri("/me")
                .header("Authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(me.status(), StatusCode::OK);
}
