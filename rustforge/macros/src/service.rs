use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

pub fn expand(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let name = &input.ident;

    let expanded = quote! {
        #input

        impl #name {
            /// Enregistrement DI compile-time (singleton typé, pas de factory `dyn`).
            pub fn register_provider(container: &::rustforge::di::Container) {
                container.register_default::<#name>();
            }
        }
    };

    expanded.into()
}
