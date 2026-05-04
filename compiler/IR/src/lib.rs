mod builder;
mod constant;
mod context;
mod data_layout;
mod emitable;
mod global;
pub mod instruction;
mod module;
mod types;
mod value;

pub use self::{
  builder::Builder,
  constant::{Constant as IRConstant, Data as ConstantData},
  context::{Context, Session},
  data_layout::{DataLayout, SymbolDecoration, TypeSpecs},
  global::{
    BasicBlock, Function as IRFunction, Global as GlobalValue,
    Initializer as IRStaticInitializer, Linkage, Variable as IRVariable,
  },
  module::Module,
  types::{Type, TypeRef, TypeRefMut},
  value::{Arguments as IRArguments, Data as ValueData, Value, ValueID},
};
