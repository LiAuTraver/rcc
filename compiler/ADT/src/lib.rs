#![feature(exact_div)]
// const for Box::new
#![feature(const_convert)]
// for `impl const` traits
#![feature(const_trait_impl)]
#![feature(derive_const)]
#![feature(const_clone)]
#![feature(const_cmp)]
#![feature(const_try)]
#![feature(const_ops)]
// NTTP
#![feature(adt_const_params)]
#![feature(const_default)]
// clippy mistakes
#![allow(unused_features)]
#![feature(const_result_unwrap_unchecked)]
mod floating;
mod integral;
mod uom;

pub use self::{
  floating::{Floating, Format as FloatFormat},
  integral::{Integral, Signedness},
  uom::{Alignment, Size, SizeBit},
};
