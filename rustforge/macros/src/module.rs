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

    let export_names: Vec<_> = args.exports.iter().map(|e| quote! { stringify!(#e) }).collect();

    let expanded = quote! {
        #input

        impl #module_name {
            /// Monte tous les contrôleurs du module sur un `Router` Axum (compile-time).
            pub fn wire_routes(
                router: ::rustforge::http::axum::Router,
                container: &::rustforge::di::Container,
            ) -> Result<::rustforge::http::axum::Router, ::rustforge::di::DiError> {
                let mut router = router;
                #(#controller_wires)*
                Ok(router)
            }
        }

        impl ::rustforge::core::Module for #module_name {
            fn metadata(&self) -> ::rustforge::core::ModuleMetadata {
                ::rustforge::core::ModuleMetadata {
                    imports: vec![],
                    providers: vec![#(#provider_registers),*],
                    exports: vec![#(#export_names),*],
                }
            }
        }

        impl ::rustforge::core::HttpModule for #module_name {}

        impl ::rustforge::ModuleWireRoutes for #module_name {
            fn wire_routes(
                router: ::rustforge::http::axum::Router,
                container: &::rustforge::di::Container,
            ) -> Result<::rustforge::http::axum::Router, ::rustforge::di::DiError> {
                #module_name::wire_routes(router, container)
            }
        }
    };

    let _ = args.imports;
    expanded.into()
}
