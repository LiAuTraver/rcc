use ::rcc_adt::Size;
use ::rcc_ast::{
  Context, VarDeclKind,
  types::{FunctionSpecifier, QualifiedType, Type},
};
use ::rcc_memory::{
  ArenaVec, CollectIn, IntrusiveRedeclarableLink,
  make_intrusive_redeclarable_node,
};
use ::rcc_shared::{Linkage, SourceSpan, Storage};
use ::rcc_utils::{StrRef, ensure_is_pod, interconvert, make_trio_for};
use ::std::{cell::Cell, ops::Deref};

use crate::{expression::ExprRef, statement::Compound};

#[derive(Debug)]
pub struct TranslationUnit<'c> {
  pub declarations: &'c [DeclRef<'c>],
}

pub type DeclRef<'c> = &'c ExternalDeclaration<'c>;

/// Size: probably 19 * sizeof(usize)
#[repr(C)]
#[derive(Debug)]
pub struct ExternalDeclaration<'c> {
  __intrusive_redeclarable_link: IntrusiveRedeclarableLink<Self>, // 2 ptr.
  pub name: StrRef<'c>,                                           // 2 ptr
  pub qualified_type: QualifiedType<'c>,                          // 2 ptr
  pub declaration_data: DeclarationData<'c>,                      // 11 ptr
  pub linkage: Linkage,                                           // packed
  /// the complex rule of [`Storage`] and [`VarDeclKind`] are managed by [`Linkage`].
  pub declkind: VarDeclKind, // packed
  pub span: SourceSpan,                                           // 1+ ptr
}

make_intrusive_redeclarable_node!(
  __intrusive_redeclarable_link => pub ExternalDeclaration[
    name: StrRef<'c>,
    qualified_type: QualifiedType<'c>,
    linkage: Linkage,
    declkind: VarDeclKind,
    declaration_data: DeclarationData<'c>,
    span: SourceSpan
]: 'c
);
impl<'c> ExternalDeclaration<'c> {
  #[inline]
  pub fn definition(&self) -> Option<&Self> {
    self.iter().find(|decl| decl.is_definition())
  }
}
make_trio_for!(Function, DeclarationData, 'c, Function);
make_trio_for!(VarDef, DeclarationData, 'c, Variable);
make_trio_for!(Typedef, DeclarationData,'c, Typedef);
interconvert!(Function, DeclarationData, 'c, Function);
interconvert!(VarDef, DeclarationData, 'c, Variable);
interconvert!(Typedef, DeclarationData, 'c, Typedef);
impl<'c> Deref for ExternalDeclaration<'c> {
  type Target = DeclarationData<'c>;

  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    &self.declaration_data
  }
}
#[derive(Debug)]
pub enum DeclarationData<'c> {
  Function(Function<'c>),
  Variable(VarDef<'c>),
  Typedef(Typedef<'c>),
}
impl<'c> DeclarationData<'c> {
  #[must_use]
  #[inline(always)]
  pub fn is_definition(&self) -> bool {
    ::rcc_utils::static_dispatch!(
      self,
      |variant| variant.is_definition() =>
      Function Variable Typedef
    )
  }
}
#[derive(Debug)]
pub struct Function<'c> {
  pub specifier: FunctionSpecifier,
  /// in C: must be variable and has no initializer.
  pub parameters: &'c [DeclRef<'c>],
  pub body: Cell<Option<Compound<'c>>>,
  pub labels: Cell<&'c [StrRef<'c>]>,
  pub gotos: Cell<&'c [StrRef<'c>]>,
}
impl<'c> Function<'c> {
  #[must_use]
  #[inline(always)]
  pub fn is_definition(&self) -> bool {
    self.body.get().is_some()
  }

  #[inline]
  pub fn new(
    specifier: FunctionSpecifier,
    parameters: &'c [DeclRef<'c>],
    body: Option<Compound<'c>>,
  ) -> Self {
    Self {
      specifier,
      parameters,
      body: Cell::new(body),
      labels: Cell::default(),
      gotos: Cell::default(),
    }
  }

  #[inline]
  pub fn new_decl(
    specifier: FunctionSpecifier,
    parameters: &'c [DeclRef<'c>],
  ) -> Self {
    Self::new(specifier, parameters, None)
  }
}
#[derive(Debug)]
pub struct VarDef<'c> {
  pub is_named_constant: bool,
  pub storage: Storage,
  pub initializer: Option<Initializer<'c>>,
}
impl<'c> VarDef<'c> {
  #[must_use]
  #[inline(always)]
  pub fn is_definition(&self) -> bool {
    self.initializer.is_some()
  }

  #[inline]
  pub fn new(
    initializer: Option<Initializer<'c>>,
    is_named_constant: bool,
    storage: Storage,
  ) -> Self {
    Self {
      initializer,
      is_named_constant,
      storage,
    }
  }

  #[inline]
  pub const fn decl(storage: Storage) -> Self {
    Self {
      initializer: None,
      is_named_constant: false,
      storage,
    }
  }

  #[inline]
  pub fn def(
    initializer: Initializer<'c>,
    is_named_constant: bool,
    storage: Storage,
  ) -> Self {
    Self {
      initializer: Some(initializer),
      is_named_constant,
      storage,
    }
  }
}
#[derive(Debug, Default)]
pub struct Typedef<'c> {
  _idfk: ::std::marker::PhantomData<&'c str>,
}
impl Typedef<'_> {
  #[inline(always)]
  pub const fn new() -> Self {
    Self {
      _idfk: ::std::marker::PhantomData,
    }
  }

  #[inline(always)]
  pub const fn is_definition(&self) -> bool {
    false
  }
}
#[derive(Debug)]
pub enum Initializer<'c> {
  /// fixme: dont do [`ExprRef`] here but store a real expr so that we have cache locality.
  ///
  /// FAILED: Don't try to do this once the 1k LOC is done...
  Scalar(ExprRef<'c>),
  List(InitializerList<'c>),
}
interconvert!(ExprRef, Initializer, 'c, Scalar);
interconvert!(InitializerList, Initializer, 'c, List);
#[derive(Debug)]
pub struct InitializerList<'c> {
  pub entries: &'c [InitializerListEntry<'c>],
  pub span: SourceSpan,
}

#[derive(Debug)]
pub struct InitializerListEntry<'c> {
  pub designator: Designator<'c>,
  pub initializer: Initializer<'c>,
  pub is_implicit: bool,
}

impl<'c> InitializerListEntry<'c> {
  pub fn new(
    designators: Designator<'c>,
    initializer: Initializer<'c>,
    is_implicit: bool,
  ) -> Self {
    Self {
      designator: designators,
      initializer,
      is_implicit,
    }
  }
}
/// currently an alias of [`Size`]. no intention to make it a new type; but for clarity.
pub type Index = Size;
#[derive(Debug)]
pub enum Designator<'c> {
  Array(Index),
  Field(
    /* Field iterator or so.. */ ::std::marker::PhantomData<&'c u8>,
  ),
}

impl Designator<'_> {
  /// as error node. you can't use [`usize::MAX`] in subscript
  /// since it would require the array has [`usize::MAX`] + 1 length, which is impossible.
  /// Also nobody in SANE would allocate such a huge array...
  #[allow(non_upper_case_globals)]
  pub const npos: Index = Index::MAX;
  // pub const nofield...
}

ensure_is_pod!(Initializer<'_>);
ensure_is_pod!(VarDef<'_>);
ensure_is_pod!(Function<'_>);
ensure_is_pod!(DeclRef<'_>);
ensure_is_pod!(TranslationUnit<'_>);

impl<'c> TranslationUnit<'c> {
  pub fn new(
    context: &Context<'c>,
    declarations: impl IntoIterator<Item = DeclRef<'c>>,
  ) -> Self {
    Self {
      declarations: declarations
        .into_iter()
        .collect_in::<ArenaVec<_>>(context.arena())
        .into_bump_slice(),
    }
  }
}

impl<'c> Initializer<'c> {
  pub fn span(&self) -> SourceSpan {
    ::rcc_utils::static_dispatch!(
      self,
      |variant| variant.span() =>
      Scalar List
    )
  }
}
impl<'c> InitializerList<'c> {
  pub fn new(
    entries: &'c [InitializerListEntry<'c>],
    span: SourceSpan,
  ) -> Self {
    Self { entries, span }
  }

  fn span(&self) -> SourceSpan {
    self.span
  }

  #[inline]
  pub fn is_empty(&self) -> bool {
    self.entries.is_empty()
  }

  #[inline]
  pub fn len(&self) -> usize {
    self.entries.len()
  }
}

mod fmt {
  use ::std::fmt::Display;

  use super::*;

  impl<'c> Display for TranslationUnit<'c> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
      self
        .declarations
        .iter()
        .try_for_each(|decl| writeln!(f, "{}", decl))
    }
  }

  impl<'c> Display for ExternalDeclaration<'c> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
      match &self.declaration_data {
        DeclarationData::Function(function) => match *self.qualified_type {
          Type::FunctionProto(proto) => {
            write!(f, "{} {}(", proto.return_type, self.name)?;
            for (index, param) in function.parameters.iter().enumerate() {
              if index > 0 {
                write!(f, ", ")?;
              }
              write!(f, "{}", param)?;
            }
            write!(f, ")")?;
            write!(f, " {}", function)
          },
          // _ => write!(f, "{} {} {}", self.qualified_type, self.name, function),
          _ => unreachable!("can we reach here fr?"),
        },
        DeclarationData::Variable(var_def) => write!(
          f,
          "{} {} {} {}",
          var_def.storage, self.qualified_type, self.name, var_def
        ),
        DeclarationData::Typedef(_) =>
          write!(f, "typedef {} {}", self.qualified_type, self.name),
      }
    }
  }

  impl<'c> Display for Function<'c> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
      if let Some(body) = &self.body.get() {
        write!(f, "{}", body)
      } else {
        write!(f, ";")
      }
    }
  }

  impl<'c> Display for VarDef<'c> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
      if let Some(initializer) = &self.initializer {
        write!(f, "= {}", initializer)?;
      }
      write!(f, ";")
    }
  }

  impl<'c> Display for Initializer<'c> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
      ::rcc_utils::static_dispatch!(
        self,
        |variant| variant.fmt(f) =>
        Scalar List
      )
    }
  }

  impl<'c> Display for InitializerList<'c> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{{")?;
      if !self.entries.is_empty() {
        write!(f, " ")?;
      }

      for (i, entry) in self.entries.iter().enumerate() {
        if i > 0 {
          write!(f, ", ")?;
        }

        match entry.designator {
          Designator::Array(index) => write!(f, "[{}]", index)?,
          Designator::Field(_) => write!(f, ".<field>")?,
        }

        write!(f, " = {}", entry.initializer)?;
      }

      if !self.entries.is_empty() {
        write!(f, " ")?;
      }
      write!(f, "}}")
    }
  }
}
