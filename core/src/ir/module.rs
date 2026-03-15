use ::slotmap::SlotMap;

use super::{
  Constant, Lookup,
  value::{ValueData, ValueID},
};
use crate::common::StrRef;

#[derive(Debug, Default)]
pub struct Module {
  /// global function and variable entry.
  pub globals: Vec<ValueID>,
}

/// **Global** function in TAC-SSA form
#[derive(Debug)]
pub struct Function<'context> {
  pub name: StrRef<'context>,
  pub params: Vec<ValueID>,
  pub blocks: Vec<ValueID>,
  pub is_variadic: bool,
}

impl<'context> Function<'context> {
  pub fn new(
    name: StrRef<'context>,
    params: Vec<ValueID>,
    blocks: Vec<ValueID>,
    is_variadic: bool,
  ) -> Self {
    Self {
      name,
      params,
      blocks,
      is_variadic,
    }
  }

  pub fn new_empty(name: StrRef<'context>, is_variadic: bool) -> Self {
    Self {
      name,
      is_variadic,
      params: Default::default(),
      blocks: Default::default(),
    }
  }

  #[inline(always)]
  pub fn is_definition(&self) -> bool {
    !self.blocks.is_empty()
  }
}

/// **Global** variable.
#[derive(Debug)]
pub struct Variable<'context> {
  pub name: StrRef<'context>,
  pub initializer: Option<Initializer<'context>>,
}

impl<'context> Variable<'context> {
  pub fn new(
    name: StrRef<'context>,
    initializer: Option<Initializer<'context>>,
  ) -> Self {
    Self { name, initializer }
  }
}

/// type should always be [`super::Type::Label`].
#[derive(Debug, Default)]
pub struct BasicBlock {
  pub instructions: Vec<ValueID>,
  pub terminator: ValueID,
}

impl BasicBlock {
  pub fn new(instructions: Vec<ValueID>, terminator: ValueID) -> Self {
    Self {
      instructions,
      terminator,
    }
  }
}

#[derive(Debug)]
pub struct Argument {
  pub function: ValueID,
  pub index: usize,
}

impl Argument {
  pub fn new(function: ValueID, index: usize) -> Self {
    Self { function, index }
  }
}

/// **Static** initializer.
#[derive(Debug, Clone)]
pub enum Initializer<'context> {
  Scalar(Constant<'context>),
  Aggregate(Vec<Initializer<'context>>),
}
impl Clone for Constant<'_> {
  fn clone(&self) -> Self {
    Self::new(self.value.clone(), self.span)
  }
}
