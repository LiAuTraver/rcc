use crate::{
  common::environment::{Environment, Symbol, SymbolRef},
  parser::declaration::Program,
};

pub mod analyzer;
pub mod conversion;
pub mod declaration;
pub mod expression;
pub mod statement;

#[derive(Debug)]
pub struct Analyzer {
  program: Program,
  environment: Environment,
  current_function: Option<SymbolRef>,
  errors: Vec<String>,
  warnings: Vec<String>,
}
