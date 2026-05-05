use ::rcc_ast::Constant;
use ::rcc_shared::Storage;
use ::rcc_utils::StrRef;

use super::ValueID;

#[derive(Debug, Clone, Copy)]
pub enum Linkage {
  /// Visible to other translation units, [`Storage::Extern`].
  External,
  /// Not visible, [`Storage::Static`].
  Internal,
  /// Anonymous data like string literals and const arrays. Also [`Storage::Static`].
  Private,
  // /// Tentative.
  // Common,
}

impl From<Storage> for Linkage {
  fn from(storage: Storage) -> Self {
    use Linkage::*;
    use Storage::*;
    match storage {
      Extern => External,
      Static => Internal,
      _ => panic!("not a truly storage class."),
    }
  }
}
// impl Linkage {
//   pub fn maybe_tentative(decl: DeclRef) -> Self {
//     use Linkage::*;
//     use Storage::*;
//     use VarDeclKind::*;
//     match (decl.storage_class(), decl.declkind()) {
//       (_, Tentative) => Common,
//       (Extern, _) => External,
//       (Static, _) => Internal,
//       _ => panic!("invalid call"),
//     }
//   }
// }
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
  pub linkage: Linkage,
  /// Shall be [`Argument`].
  pub params: Vec<ValueID>,
  /// Shall be [`BasicBlock`].
  pub blocks: Vec<ValueID>,
  pub is_variadic: bool,
}

impl<'c> Function<'c> {
  pub fn new_empty(
    name: StrRef<'c>,
    linkage: Linkage,
    params: Vec<ValueID>,
    is_variadic: bool,
  ) -> Self {
    Self {
      name,
      linkage,
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
  pub linkage: Linkage,
  pub initializer: Option<Initializer<'c>>,
}

impl<'c> Variable<'c> {
  /// the `linkage` param has some nuances and UB w.r.t. C standard if tentative definition is involved.
  pub fn new(
    name: StrRef<'c>,
    linkage: Linkage,
    initializer: Option<Initializer<'c>>,
  ) -> Self {
    Self {
      name,
      linkage,
      initializer,
    }
  }
}

impl Variable<'_> {
  pub fn is_definition(&self) -> bool {
    // use Linkage::*;
    self.initializer.is_some()
    // || matches!(self.linkage, Common)
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
  Zeroed(),
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
