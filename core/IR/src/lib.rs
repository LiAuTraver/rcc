#[macro_use]
mod emitter;
mod context;
mod emitable;
mod fmt;
mod instruction;
mod module;
mod printer;
mod types;
mod value;

pub use self::{
  context::{Context, Session},
  emitter::Emitter,
  module::{
    Argument, BasicBlock, Function as IRFunction,
    Initializer as IRStaticInitializer, Module, Variable as IRGlobalValue,
  },
  printer::{IRPrinter, Printable, Printer},
  types::{Type, TypeRef, TypeRefMut},
  value::{Data as ValueData, Value, ValueID},
};
