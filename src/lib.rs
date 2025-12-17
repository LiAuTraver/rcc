#![allow(internal_features)]
// for using core::intrinsics::breakpoint
#![feature(core_intrinsics)]

pub(crate) mod common;
pub mod lexer;
pub mod parser;
pub mod utils;
