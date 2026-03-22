#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![feature(unsized_const_params)]

mod colored;
mod out;
pub use self::{
  colored::{FlushOnDropRAII, Palette, StickyWriter},
  out::Default as TreeDumper,
};
