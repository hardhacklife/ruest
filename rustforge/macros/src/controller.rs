use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct, LitStr, Type};

pub fn expand(attr: TokenStream, item: TokenStream) -> TokenStream {
    let prefix = parse_macro_input!(attr as LitStr);
    let prefix_val = prefix.value();
    let input = parse_macro_input!(item as ItemStruct);
    let name = &input.ident;

    let fields = match &input.fields {
        syn::Fields::Named(fields) => &fields.named,
        _ => {
            return syn::Error::new_spanned(&input, "controller struct must have named fields")
                .to_compile_error()
                .into();
        }
    };

    let mut field_inits = Vec::new();

    for field in fields {
        let field_name = field.ident.as_ref().unwrap();

        if is_inject_type(&field.ty) {
            field_inits.push(quote! {
                #field_name: ::rustforge::di::Inject::resolve(container)?
            });
        } else {
            field_inits.push(quote! {
                #field_name: Default::default()
            });
        }
    }

    let expanded = quote! {
        #input

        impl #name {
            /// Préfixe HTTP (littéral compile-time).
            pub const PREFIX: &'static str = #prefix;

            #[doc(hidden)]
            pub fn prefix() -> &'static str {
                Self::PREFIX
            }

            /// Instancie le contrôleur via DI typée (`get::<T>()` monomorphisé).
            pub fn from_container(
                container: &::rustforge::di::Container,
            ) -> Result<Self, ::rustforge::di::DiError> {
                Ok(Self {
                    #(#field_inits,)*
                })
            }
        }
    };

    let _ = prefix_val;
    expanded.into()
}

fn is_inject_type(ty: &Type) -> bool {
    if let Type::Path(path) = ty {
        path.path
            .segments
            .last()
            .map(|s| s.ident == "Inject")
            .unwrap_or(false)
    } else {
        false
    }
}
