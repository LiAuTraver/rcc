use crate::{common::environment::Environment, parser::declaration::Program};

pub mod analyzer;
pub mod declaration;
pub mod expression;
pub mod statement;
pub mod conversion;

#[derive(Debug)]
pub struct Analyzer {
  program: Program,
  environment: Environment,
  errors: Vec<String>,
  warnings: Vec<String>,
}
