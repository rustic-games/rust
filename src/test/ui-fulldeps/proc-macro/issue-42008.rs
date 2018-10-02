// aux-build:issue-42008.rs

#![feature(proc_macro_non_items)]

#[macro_use]
extern crate issue_42008;

fn main() {
    let _ = foo!("");
}
