#![allow(unused)]
mod builder;
mod context;
mod fmt;
mod instruction;
mod module;
mod types;
mod value;

pub use self::{
  builder::ModuleBuilder,
  context::Context,
  module::{
    BasicBlock, Function as IRFunction, Initializer as IRStaticInitializer,
    Module, Variable as IRGlobalValue,
  },
  types::{Type, TypeRef, TypeRefMut},
};
