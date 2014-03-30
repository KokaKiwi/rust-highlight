#![crate_id = "highlight#0.1.0"]
#![crate_type = "dylib"]
#![license = "MIT"]

extern crate syntax;
extern crate serialize;
extern crate collections;

pub mod core;
pub mod colors;
pub mod backend;
