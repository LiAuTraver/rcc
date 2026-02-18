#![allow(unused)]
mod builder;
pub mod instruction;
mod module;

pub use self::{
  builder::ModuleBuilder,
  module::{
    BasicBlock, Function as IRFunction, Initializer as IRStaticInitializer,
    Module, Variable as IRGlobalValue, ilist_type,
  },
};
