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

mod floating;
mod integral;

pub use self::{
  floating::{Floating, Format as FloatFormat},
  integral::{Integral, Signedness},
};
