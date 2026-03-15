#![allow(unused)]
mod builder;
mod context;
mod dump;
mod fmt;
mod instruction;
mod module;
mod types;
mod value;

use self::value::Lookup;
type Constant<'context> = crate::sema::expression::Constant<'context>;
pub use self::{
  builder::ModuleBuilder,
  context::Context,
  dump::IRDumper,
  module::{
    Argument, BasicBlock, Function as IRFunction,
    Initializer as IRStaticInitializer, Module, Variable as IRGlobalValue,
  },
  types::{Type, TypeRef, TypeRefMut},
  value::{Data as ValueData, Value, ValueID},
};
