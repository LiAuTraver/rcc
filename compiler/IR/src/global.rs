use ::rcc_ast::Constant;
use ::rcc_utils::StrRef;

use super::ValueID;

#[derive(Debug)]
pub enum Linkage {
  External,
  Internal,
  Common,
}

#[derive(Debug)]
pub enum Global<'ir> {
  Function(Function<'ir>),
  Variable(Variable<'ir>),
}

impl<'ir> Global<'ir> {
  pub fn name(&self) -> StrRef<'ir> {
    ::rcc_utils::static_dispatch!(
      Global : self,
      |variant| variant.name =>
      Function Variable
    )
  }
}

#[derive(Debug)]
pub struct Function<'c> {
  pub name: StrRef<'c>,
  /// Shall be [`Argument`].
  pub params: Vec<ValueID>,
  /// Shall be [`BasicBlock`].
  pub blocks: Vec<ValueID>,
  pub is_variadic: bool,
}

impl<'c> Function<'c> {
  pub fn new_empty(
    name: StrRef<'c>,
    params: Vec<ValueID>,
    is_variadic: bool,
  ) -> Self {
    Self {
      name,
      is_variadic,
      params,
      blocks: Default::default(),
    }
  }

  #[inline(always)]
  pub fn is_definition(&self) -> bool {
    !self.blocks.is_empty()
  }

  #[inline(always)]
  pub fn entry(&self) -> ValueID {
    self.blocks.first().copied().unwrap_or(ValueID::null())
  }
}

/// **Global** variable.
#[derive(Debug)]
pub struct Variable<'c> {
  pub name: StrRef<'c>,
  pub initializer: Option<Initializer<'c>>,
}

impl<'c> Variable<'c> {
  pub fn new(name: StrRef<'c>, initializer: Option<Initializer<'c>>) -> Self {
    Self { name, initializer }
  }
}

/// type should always be [`super::Type::Label`].
#[derive(Debug, Default)]
pub struct BasicBlock {
  /// Shall be [`super::instruction::Instruction`].
  pub instructions: Vec<ValueID>,
  /// Shall be [`super::instruction::Terminator`].
  pub terminator: ValueID,
}

impl BasicBlock {
  pub fn new(instructions: Vec<ValueID>, terminator: ValueID) -> Self {
    Self {
      instructions,
      terminator,
    }
  }

  pub fn is_empty(&self) -> bool {
    self.instructions.is_empty() && self.terminator.is_null()
  }
}

/// **Static** initializer.
#[derive(Debug, Clone)]
pub enum Initializer<'c> {
  Scalar(Constant<'c>),
  Aggregate(Vec<Initializer<'c>>),
}

mod fmt {
  use ::std::fmt::{Display, Formatter, Result};

  use super::*;

  impl Display for Global<'_> {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result {
      unreachable!("when is it possible to reach here?")
    }
  }
}
