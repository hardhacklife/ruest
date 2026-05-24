//! Tests « piliers » : le framework reste modulaire, typé, sécurisé et branché de bout en bout.

use ruest::core::bootstrap;
use ruest::di::Container;
use ruest::prelude::*;
use ruest::security::{Guard, JwtGuard, SecurityConfig};

// --- Modulaire : imports enregistrent les providers enfants ---

#[derive(Debug, Default)]
struct LeafSvc;

fn register_leaf(c: &Container) {
    c.register_default::<LeafSvc>();
}

struct LeafMod;

impl ruest::core::Module for LeafMod {
    fn metadata(&self) -> ruest::core::ModuleMetadata {
        ruest::core::ModuleMetadata {
            imports: vec![],
            providers: vec![register_leaf],
            exports: vec![],
        }
    }
}

struct RootMod;

impl ruest::core::Module for RootMod {
    fn metadata(&self) -> ruest::core::ModuleMetadata {
        ruest::core::ModuleMetadata {
            imports: vec![Box::new(LeafMod)],
            providers: vec![],
            exports: vec![],
        }
    }
}

#[test]
fn pillar_modular_imports_wire_di() {
    let app = bootstrap(RootMod).expect("bootstrap");
    assert!(app.container.get::<LeafSvc>().is_ok());
}

// --- Typé : DI + AppResult + ruest_err ---

#[test]
fn pillar_typed_di_not_found_is_explicit() {
    let c = Container::new();
    let err = match c.get::<LeafSvc>() {
        Err(e) => e.to_string(),
        Ok(_) => panic!("expected not registered"),
    };
    assert!(err.contains("LeafSvc") || err.contains("not registered"));
}

#[test]
fn pillar_typed_app_error_status() {
    let e = ruest_err!(Forbidden, "admin only");
    assert_eq!(e.status(), ruest::http::axum::http::StatusCode::FORBIDDEN);
}

// --- Sécurisé : JWT + guards compilables ---

#[tokio::test]
async fn pillar_security_jwt_guard() {
    let guard = JwtGuard;
    assert!(guard.can_activate(None).await.is_err());
}

#[test]
fn pillar_security_config_dev_is_constructible() {
    let cfg = SecurityConfig::dev();
    assert!(cfg.is_public_route("/health"));
    assert!(!cfg.is_public_route("/customers/"));
}

// --- Fonctionnel : stack minimal bootstrap + module macro ---

#[service]
#[derive(Default)]
struct PingSvc;

#[controller("/ping")]
struct PingCtl {
    svc: Inject<PingSvc>,
}

#[routes]
impl PingCtl {
    #[get("/")]
    async fn ping(&self) -> AppResult<Json<serde_json::Value>> {
        Ok(Json(serde_json::json!({ "ok": true })))
    }
}

#[module(controllers = [PingCtl], providers = [PingSvc])]
struct PingMod;

#[module(imports = [PingMod])]
struct AppViaImports;

#[test]
fn pillar_functional_bootstrap_and_imports() {
    let b = bootstrap_app(AppViaImports).expect("bootstrap_app");
    assert!(b.app.container.get::<PingSvc>().is_ok());
}

#[tokio::test]
async fn pillar_functional_http_route() {
    use ruest::http::axum::body::Body;
    use ruest::http::axum::http::Request;
    use tower::ServiceExt;

    let builder = bootstrap_app(AppViaImports).expect("bootstrap");
    let router = AppViaImports::wire_routes(
        ruest::http::axum::Router::new(),
        &builder.app.container,
    )
    .expect("wire");

    let res = router
        .oneshot(
            Request::builder()
                .uri("/ping/")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert!(res.status().is_success());
}
