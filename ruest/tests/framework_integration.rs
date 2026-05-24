//! Tests d'intégration : macros `#[module]` / `#[service]` / `#[routes]` + bootstrap + HTTP.

use ruest::http::axum::body::Body;
use ruest::http::axum::http::{Request, StatusCode};
use ruest::http::axum::Router;
use ruest::core::bootstrap;
use ruest::prelude::*;
use ruest::testing::TestFactory;
use tower::ServiceExt;

#[service]
#[derive(Default)]
struct PingService {
    hits: u32,
}

impl PingService {
    fn label(&self) -> &'static str {
        "pong"
    }
}

#[controller("/ping")]
struct PingController {
    service: Inject<PingService>,
}

#[routes]
impl PingController {
    #[get("/")]
    async fn index(&self) -> AppResult<Json<serde_json::Value>> {
        Ok(Json(serde_json::json!({
            "message": self.service.label(),
            "hits": self.service.hits,
        })))
    }
}

#[module(controllers = [PingController], providers = [PingService])]
struct PingModule;

#[module(imports = [PingModule])]
struct RootModule;

#[test]
fn bootstrap_app_resolves_imported_providers() {
    let builder = bootstrap_app(RootModule).expect("bootstrap_app");
    assert_eq!(builder.app.port, 3000);

    let svc = builder
        .app
        .container
        .get::<PingService>()
        .expect("PingService via imports");
    assert_eq!(svc.hits, 0);
}

#[test]
fn test_factory_matches_bootstrap() {
    let via_factory = TestFactory::create(RootModule).expect("TestFactory");
    let via_bootstrap = bootstrap(RootModule).expect("bootstrap");
    assert!(via_factory.container.get::<PingService>().is_ok());
    assert!(via_bootstrap.container.get::<PingService>().is_ok());
}

#[test]
fn wire_routes_delegates_to_imported_module() {
    let app = bootstrap(RootModule).expect("bootstrap");
    let router = RootModule::wire_routes(Router::new(), &app.container).expect("wire");

    let controller = PingController::from_container(&app.container).expect("from_container");
    assert_eq!(PingController::PREFIX, "/ping");
    assert_eq!(controller.service.label(), "pong");

    let _ = router;
}

#[tokio::test]
async fn http_get_ping_returns_json() {
    let builder = bootstrap_app(RootModule).expect("bootstrap");
    let router = RootModule::wire_routes(Router::new(), &builder.app.container).expect("wire");

    let response = router
        .oneshot(
            Request::builder()
                .uri("/ping/")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = ruest::http::axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["message"], "pong");
    assert_eq!(json["hits"], 0);
}

#[test]
fn ruest_err_builds_app_error() {
    let err = ruest_err!(NotFound, "absent");
    assert_eq!(err.status(), StatusCode::NOT_FOUND);
}
