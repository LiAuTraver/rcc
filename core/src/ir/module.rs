use ::slotmap::SlotMap;

use super::{
  Lookup,
  value::{ValueData, ValueID},
};
use crate::{common::StrRef, types::Constant};

#[derive(Debug, Default)]
pub struct Module<'context> {
  pub values: SlotMap<ValueID, ValueData<'context>>,
  pub globals: Vec<ValueID>, // global function and variable entry.
}

impl<'a> Module<'a> {
  #[inline(always)]
  pub fn insert(&mut self, value: ValueData<'a>) -> ValueID {
    self.values.insert(value)
  }
}

impl<'a> Lookup<ValueID, ValueData<'a>> for Module<'a> {
  fn lookup(&self, key: ValueID) -> &ValueData<'a> {
    &self.values[key]
  }
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

/// currently no use, just a atag.
pub trait Global {}
impl Global for Variable<'_> {}
impl Global for Function<'_> {}

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
