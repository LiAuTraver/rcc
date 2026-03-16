#[macro_use]
mod builder;
mod context;
mod dump;
mod emit;
mod fmt;
mod instruction;
mod module;
mod types;
mod value;

type Constant<'context> = crate::sema::expression::Constant<'context>;
pub use self::{
  builder::Emitter,
  context::Context,
  dump::IRDumper,
  module::{
    Argument, BasicBlock, Function as IRFunction,
    Initializer as IRStaticInitializer, Module, Variable as IRGlobalValue,
  },
  types::{Type, TypeRef, TypeRefMut},
  value::{Data as ValueData, Value, ValueID},
};
