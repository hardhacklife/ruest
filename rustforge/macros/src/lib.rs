//! Procedural macros for RustForge.

mod controller;
mod guard;
mod module;
mod route;
mod service;

use proc_macro::TokenStream;

/// Marks a struct as an injectable service.
#[proc_macro_attribute]
pub fn service(_attr: TokenStream, item: TokenStream) -> TokenStream {
    service::expand(item)
}

/// Declares a NestJS-style module.
///
/// ```ignore
/// #[module(controllers = [UserController], providers = [UserService])]
/// pub struct UserModule;
/// ```
#[proc_macro_attribute]
pub fn module(attr: TokenStream, item: TokenStream) -> TokenStream {
    module::expand(attr, item)
}

/// Marks a struct as an HTTP controller with a path prefix.
#[proc_macro_attribute]
pub fn controller(attr: TokenStream, item: TokenStream) -> TokenStream {
    controller::expand(attr, item)
}

/// Registers route handlers on a controller `impl` block.
#[proc_macro_attribute]
pub fn routes(_attr: TokenStream, item: TokenStream) -> TokenStream {
    route::expand_routes_impl(item)
}

#[proc_macro_attribute]
pub fn get(attr: TokenStream, item: TokenStream) -> TokenStream {
    route::passthrough_route_method(attr, item)
}

#[proc_macro_attribute]
pub fn post(attr: TokenStream, item: TokenStream) -> TokenStream {
    route::passthrough_route_method(attr, item)
}

#[proc_macro_attribute]
pub fn put(attr: TokenStream, item: TokenStream) -> TokenStream {
    route::passthrough_route_method(attr, item)
}

#[proc_macro_attribute]
pub fn patch(attr: TokenStream, item: TokenStream) -> TokenStream {
    route::passthrough_route_method(attr, item)
}

#[proc_macro_attribute]
pub fn delete(attr: TokenStream, item: TokenStream) -> TokenStream {
    route::passthrough_route_method(attr, item)
}

/// Marks an async function as middleware (MVP: passthrough).
#[proc_macro_attribute]
pub fn middleware(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Marks a struct as a DTO (MVP: passthrough).
#[proc_macro_attribute]
pub fn dto(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Déclare une garde d'autorisation (`impl Guard`).
///
/// ```ignore
/// #[guard(roles = ["admin"])]
/// pub struct AdminGuard;
/// ```
#[proc_macro_attribute]
pub fn guard(attr: TokenStream, item: TokenStream) -> TokenStream {
    guard::expand(attr, item)
}
