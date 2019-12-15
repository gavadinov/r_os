extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn};
use quote::quote;

#[proc_macro_attribute]
pub fn ros_test(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(item as ItemFn);
    let ident = &parsed.sig.ident;
    let body = &parsed.block.stmts;

    let outer = quote!(
        #[test_case]
        fn #ident() {
            serial_print!(concat!("\x1B[36m", file!(), " :\x1B[34m ", stringify!(#ident), "()\x1B[0m"));
            #(#body)*
            serial_println!("\x1B[32m ... [ok]\x1B[0m");
        }
    );

    outer.into()
}
