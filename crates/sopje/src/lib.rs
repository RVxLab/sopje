extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn sdk(_attr: TokenStream, struct_stream: TokenStream) -> TokenStream {
    let struct_ast = parse_macro_input!(struct_stream as DeriveInput);
    let struct_name = &struct_ast.ident;

    let tokens = quote! {
        #struct_ast

        impl #struct_name {
            pub fn say_hello() -> String {
                "Hello!".into()
            }
        }
    };

    tokens.into()
}
