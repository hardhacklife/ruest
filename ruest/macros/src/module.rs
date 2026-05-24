use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Ident, ItemStruct, Token};

struct ModuleArgs {
    controllers: Vec<Ident>,
    providers: Vec<Ident>,
    imports: Vec<Ident>,
    exports: Vec<Ident>,
}

impl Parse for ModuleArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut controllers = Vec::new();
        let mut providers = Vec::new();
        let mut imports = Vec::new();
        let mut exports = Vec::new();

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<Token![=]>()?;
            let content;
            syn::bracketed!(content in input);
            let list = syn::punctuated::Punctuated::<Ident, Token![,]>::parse_terminated(&content)?;
            let idents: Vec<Ident> = list.into_iter().collect();

            match key.to_string().as_str() {
                "controllers" => controllers = idents,
                "providers" => providers = idents,
                "imports" => imports = idents,
                "exports" => exports = idents,
                _ => {
                    return Err(syn::Error::new(
                        key.span(),
                        "unknown module attribute; expected controllers, providers, imports, or exports",
                    ));
                }
            }

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(Self {
            controllers,
            providers,
            imports,
            exports,
        })
    }
}

pub fn expand(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as ModuleArgs);
    let input = parse_macro_input!(item as ItemStruct);
    let module_name = &input.ident;

    let provider_registers: Vec<_> = args
        .providers
        .iter()
        .map(|p| quote! { #p::register_provider })
        .collect();

    let controller_wires: Vec<_> = args
        .controllers
        .iter()
        .map(|c| quote! { router = #c::wire(router, container)?; })
        .collect();

    let import_wires: Vec<_> = args
        .imports
        .iter()
        .map(|m| quote! { router = #m::wire_routes(router, container)?; })
        .collect();

    let import_instances: Vec<_> = args
        .imports
        .iter()
        .map(|m| quote! { Box::new(#m) as Box<dyn ::ruest::core::Module> })
        .collect();

    let export_names: Vec<_> = args.exports.iter().map(|e| quote! { stringify!(#e) }).collect();

    let expanded = quote! {
        #input

        impl #module_name {
            /// Monte ce module et ses sous-modules sur le routeur Axum.
            pub fn wire_routes(
                router: ::ruest::http::axum::Router,
                container: &::ruest::di::Container,
            ) -> Result<::ruest::http::axum::Router, ::ruest::di::DiError> {
                let mut router = router;
                #(#import_wires)*
                #(#controller_wires)*
                Ok(router)
            }
        }

        impl ::ruest::core::Module for #module_name {
            fn metadata(&self) -> ::ruest::core::ModuleMetadata {
                ::ruest::core::ModuleMetadata {
                    imports: vec![#(#import_instances),*],
                    providers: vec![#(#provider_registers),*],
                    exports: vec![#(#export_names),*],
                }
            }
        }

        impl ::ruest::core::HttpModule for #module_name {}

        impl ::ruest::ModuleWireRoutes for #module_name {
            fn wire_routes(
                router: ::ruest::http::axum::Router,
                container: &::ruest::di::Container,
            ) -> Result<::ruest::http::axum::Router, ::ruest::di::DiError> {
                #module_name::wire_routes(router, container)
            }
        }
    };

    expanded.into()
}
