// for const_eval_select
#![allow(internal_features)]
#![feature(core_intrinsics)]
#![feature(const_eval_select)]
//
// C/C++ like default initialization in struct fields
// #![feature(default_field_values)]
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
