#[macro_use]
mod builder;
mod constant;
mod context;
mod data_layout;
mod emitable;
mod global;
pub mod instruction;
mod types;
mod value;

pub use self::{
  builder::Builder,
  constant::{
    Constant as IRConstant, Data as ConstantData, Global as GlobalValue,
  },
  context::{Context, Session},
  data_layout::{DataLayout, SymbolDecoration, TypeSpecs},
  global::{
    BasicBlock, Function as IRFunction, Initializer as IRStaticInitializer,
    Module, Variable as IRVariable,
  },
  types::{Type, TypeRef, TypeRefMut},
  value::{Arguments as IRArguments, Data as ValueData, Value, ValueID},
};
