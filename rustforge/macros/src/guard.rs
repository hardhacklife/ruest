use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Expr, Ident, ItemStruct, LitStr, Token};

struct GuardAttr {
    roles: Vec<String>,
}

impl Parse for GuardAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut roles = Vec::new();

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            if key != "roles" {
                return Err(syn::Error::new(key.span(), "unknown key; expected `roles`"));
            }
            input.parse::<Token![=]>()?;
            let content;
            syn::bracketed!(content in input);
            let list = syn::punctuated::Punctuated::<Expr, Token![,]>::parse_terminated(&content)?;

            for expr in list {
                if let Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(s),
                    ..
                }) = expr
                {
                    roles.push(s.value());
                } else {
                    return Err(syn::Error::new_spanned(
                        expr,
                        "roles must be string literals, e.g. roles = [\"admin\"]",
                    ));
                }
            }

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(Self { roles })
    }
}

pub fn expand(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = if attr.is_empty() {
        GuardAttr { roles: vec![] }
    } else {
        parse_macro_input!(attr as GuardAttr)
    };

    let input = parse_macro_input!(item as ItemStruct);
    let name = &input.ident;

    let verify_body = if args.roles.is_empty() {
        quote! {
            if ctx.is_some() {
                Ok(())
            } else {
                Err(::rustforge::AppError::unauthorized(
                    concat!(stringify!(#name), " requires authentication"),
                ))
            }
        }
    } else {
        let role_refs: Vec<LitStr> = args
            .roles
            .iter()
            .map(|r| LitStr::new(r, proc_macro2::Span::call_site()))
            .collect();
        quote! {
            let Some(ctx) = ctx else {
                return Err(::rustforge::AppError::unauthorized(
                    concat!(stringify!(#name), " requires authentication"),
                ));
            };
            let required: &[&str] = &[#(#role_refs),*];
            ctx.claims().require_roles(required)
        }
    };

    let expanded = quote! {
        #input

        #[::async_trait::async_trait]
        impl ::rustforge::security::Guard for #name {
            async fn can_activate(
                &self,
                ctx: Option<&::rustforge::security::AuthContext>,
            ) -> Result<(), ::rustforge::AppError> {
                #verify_body
            }
        }
    };

    expanded.into()
}
