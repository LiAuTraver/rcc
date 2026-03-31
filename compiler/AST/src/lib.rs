#![feature(const_trait_impl)]
#![feature(const_convert)]

pub mod blueprints;
mod constant;
mod context;
mod environment;
mod session;
pub mod types;

pub use self::{
  blueprints::*,
  constant::{Constant, ConstantRef, ConstantRefMut},
  context::Context,
  environment::{
    Environment, Symbol, SymbolPtr, SymbolPtrMut, SymbolRef, UnitScope,
    VarDeclKind,
  },
  session::{Session, SessionRef},
};
