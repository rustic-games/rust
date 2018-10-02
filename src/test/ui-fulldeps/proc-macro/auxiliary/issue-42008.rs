//! Proc macro crate

// no-prefer-dynamic

#![deny(missing_docs)]
#![crate_type = "proc-macro"]

extern crate proc_macro;

use proc_macro::TokenStream;

/// foo is documented
#[proc_macro]
pub fn foo(input: TokenStream) -> TokenStream {
    input
}
