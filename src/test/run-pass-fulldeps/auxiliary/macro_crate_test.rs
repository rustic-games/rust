// Copyright 2013-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// no-prefer-dynamic

#![crate_type = "proc-macro"]
#![feature(rustc_private)]

extern crate syntax;
extern crate rustc;
extern crate rustc_plugin;
extern crate syntax_pos;
extern crate proc_macro;

use proc_macro::{TokenTree, TokenStream};

#[proc_macro_attribute]
pub fn rustc_duplicate(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut new_name = Some(attr.into_iter().nth(0).unwrap());
    let mut encountered_idents = 0;
    let input = item.to_string();
    let ret = item.into_iter().map(move |token| match token {
        TokenTree::Ident(_) if encountered_idents == 1 => {
            encountered_idents += 1;
            new_name.take().unwrap()
        }
        TokenTree::Ident(_) => {
            encountered_idents += 1;
            token
        }
        _ => token
    }).collect::<TokenStream>();
    let mut input_again = input.parse::<TokenStream>().unwrap();
    input_again.extend(ret);
    input_again
}
