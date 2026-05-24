use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ImplItem, ImplItemFn, ItemImpl, LitStr};

pub fn passthrough_route_method(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

enum HttpVerb {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

impl HttpVerb {
    fn route_fn(&self) -> proc_macro2::Ident {
        match self {
            HttpVerb::Get => quote::format_ident!("get"),
            HttpVerb::Post => quote::format_ident!("post"),
            HttpVerb::Put => quote::format_ident!("put"),
            HttpVerb::Patch => quote::format_ident!("patch"),
            HttpVerb::Delete => quote::format_ident!("delete"),
        }
    }
}

struct RouteMethod {
    verb: HttpVerb,
    path: LitStr,
    func: ImplItemFn,
}

pub fn expand_routes_impl(item: TokenStream) -> TokenStream {
    let item_impl = parse_macro_input!(item as ItemImpl);
    let self_ty = item_impl.self_ty.clone();

    let mut route_methods = Vec::new();

    for impl_item in item_impl.items {
        if let ImplItem::Fn(func) = impl_item {
            if let Some(route) = parse_route_method(&func) {
                route_methods.push(route);
            }
        }
    }

    let method_fns: Vec<_> = route_methods
        .iter()
        .map(|r| {
            let mut func = r.func.clone();
            func.attrs.retain(|a| !is_route_attr(a));
            quote! { #func }
        })
        .collect();

    let route_mounts: Vec<_> = route_methods.iter().map(|route| {
        let route_fn = route.verb.route_fn();
        let fn_name = &route.func.sig.ident;
        let suffix = &route.path;

        quote! {
            {
                let path = ::ruest::router::join_paths(Self::PREFIX, #suffix);
                router = router.route(
                path,
                ::ruest::http::axum::routing::#route_fn({
                    let controller = std::sync::Arc::clone(&controller);
                    move || {
                        let controller = std::sync::Arc::clone(&controller);
                        async move {
                            ::ruest::http::axum::response::IntoResponse::into_response(
                                controller.#fn_name().await,
                            )
                        }
                    }
                }),
            );
            }
        }
    }).collect();

    let expanded = quote! {
        impl #self_ty {
            #(#method_fns)*

            /// Monte les routes Axum (handlers monomorphisés, chemins statiques).
            pub fn mount(
                router: ::ruest::http::axum::Router,
                controller: std::sync::Arc<Self>,
            ) -> ::ruest::http::axum::Router {
                let mut router = router;
                #(#route_mounts)*
                router
            }

            /// Branche ce contrôleur sur le routeur (résolution DI typée).
            pub fn wire(
                router: ::ruest::http::axum::Router,
                container: &::ruest::di::Container,
            ) -> Result<::ruest::http::axum::Router, ::ruest::di::DiError> {
                let controller = std::sync::Arc::new(Self::from_container(container)?);
                Ok(Self::mount(router, controller))
            }
        }
    };

    expanded.into()
}

fn parse_route_method(func: &ImplItemFn) -> Option<RouteMethod> {
    for attr in &func.attrs {
        if let Some((verb, path)) = parse_route_attr(attr) {
            return Some(RouteMethod {
                verb,
                path,
                func: func.clone(),
            });
        }
    }
    None
}

fn parse_route_attr(attr: &syn::Attribute) -> Option<(HttpVerb, LitStr)> {
    let path = attr.path();
    let ident = path.segments.last()?.ident.to_string();
    let verb = match ident.as_str() {
        "get" => HttpVerb::Get,
        "post" => HttpVerb::Post,
        "put" => HttpVerb::Put,
        "patch" => HttpVerb::Patch,
        "delete" => HttpVerb::Delete,
        _ => return None,
    };
    let path: LitStr = attr.parse_args().ok()?;
    Some((verb, path))
}

fn is_route_attr(attr: &syn::Attribute) -> bool {
    parse_route_attr(attr).is_some()
}
