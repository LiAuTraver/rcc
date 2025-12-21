use crate::common::keyword::Keyword;
use crate::common::token::Literal;
use crate::common::types::Primitive;
use crate::parser::expression::Expression;
use crate::parser::statement::Compound;
use ::strum_macros::{Display, EnumString};
use std::marker::ConstParamTy;

pub struct TranslationUnit {
  pub declarations: Vec<Declaration>,
}
/// declaration:
///        declaration-specifiers init-declarator-listopt ;
///        attribute-specifier-sequence declaration-specifiers init-declarator-list ; (don't care)
///        static_assert-declaration (don't care)
///        attribute-declaration (don't care)
pub enum Declaration {
  Function(Function),
  Variable(VarDef),
}
/// storage-class-specifier
pub enum Storage {
  Automatic,
  Register,
  Extern,
  Static,
  TypeDef,     // ??? this counted as storage class?
  ThreadLocal, // I won't care about this now
  Constexpr,   // ditto
}
impl From<&Keyword> for Storage {
  fn from(kw: &Keyword) -> Self {
    match kw {
      Keyword::Auto => Storage::Automatic,
      Keyword::Register => Storage::Register,
      Keyword::Extern => Storage::Extern,
      Keyword::Static => Storage::Static,
      Keyword::Typedef => Storage::TypeDef,
      Keyword::ThreadLocal => Storage::ThreadLocal,
      // Keyword::Constexpr => Storage::Constexpr,
      _ => panic!("cannot convert {:?} to Storage", kw),
    }
  }
}
impl From<&Literal> for Storage {
  fn from(literal: &Literal) -> Self {
    match literal {
      Literal::Keyword(kw) => Storage::from(kw),
      _ => panic!("cannot convert {:?} to Storage", literal),
    }
  }
}
/// type-specifier-qualifier:
///      type-specifier
///      type-qualifier
///      alignment-specifier (don't care)
/// type-qualifier
#[derive(EnumString, Display)]
pub enum Qualifier {
  #[strum(serialize = "const")]
  Const,
  #[strum(serialize = "volatile")]
  Volatile,
  #[strum(serialize = "restrict")]
  Restrict,
  #[strum(serialize = "_Atomic")]
  #[strum(serialize = "atomic")]
  Atomic, // (don't care)
}
impl From<&Literal> for Qualifier {
  fn from(literal: &Literal) -> Self {
    match literal {
      Literal::Keyword(kw) => match kw {
        Keyword::Const => Qualifier::Const,
        Keyword::Volatile => Qualifier::Volatile,
        Keyword::Restrict => Qualifier::Restrict,
        Keyword::Atomic => Qualifier::Atomic,
        _ => panic!("cannot convert {:?} to Qualifier", kw),
      },
      _ => panic!("cannot convert {:?} to Qualifier", literal),
    }
  }
}
pub enum Modifier {
  Pointer(Vec<Qualifier>),
  Array(ArrayModifier),
  Function(FunctionSignature),
}
/// abstract declarator: no variable name/identifier
///
/// used in parsing
#[derive(ConstParamTy, PartialEq, Eq)]
pub enum DeclaratorType {
  Abstract,
  Named,
  Maybe,
}
/// declarator:
///     pointer_opt direct-declarator
/// direct-declarator:
///     ( declarator )
///     identifier attribute-specifier-sequence_opt
///     array-declarator attribute-specifier-sequence_opt
///     function-declarator attribute-specifier-sequence_opt
///
/// currently i only care about identifier and function-declarator!
pub struct Declarator {
  pub name: Option<String>,
  pub modifiers: Vec<Modifier>, // pointer, array, function
}
pub struct Member {
  pub specifiers: Vec<Specifier>,
  pub qualifiers: Vec<Qualifier>,
  pub modifiers: Vec<Modifier>,
  pub declarator: Option<Declarator>,
  pub bit_width: Option<Expression>,
}
pub struct Parameter {
  pub specifications: DeclSpecs,
  pub declarator: Declarator,
}
pub struct Struct {
  pub name: Option<String>,
  pub members: Vec<Member>,
}
/// type-specifier
#[derive(EnumString, Display)]
pub enum Specifier {
  #[strum(serialize = "void")]
  Void,
  #[strum(serialize = "char")]
  Char,
  #[strum(serialize = "short")]
  Short,
  #[strum(serialize = "int")]
  Int,
  #[strum(serialize = "long")]
  Long,
  #[strum(serialize = "float")]
  Float,
  #[strum(serialize = "double")]
  Double,
  #[strum(serialize = "signed")]
  Signed,
  #[strum(serialize = "unsigned")]
  Unsigned,
  #[strum(serialize = "_Bool")]
  #[strum(serialize = "bool")]
  Bool,
  #[strum(serialize = "_Complex")]
  #[strum(serialize = "complex")]
  Complex,
  // vvv below should be wrong, but now don't care
  #[strum(disabled)]
  Struct(Struct),
  #[strum(disabled)]
  Union(Struct),
  #[strum(disabled)]
  Enum(EnumSpecifier),
  #[strum(disabled)]
  Typedef(String),
}
impl From<&Keyword> for Specifier {
  fn from(kw: &Keyword) -> Self {
    match kw {
      Keyword::Void => Specifier::Void,
      Keyword::Char => Specifier::Char,
      Keyword::Short => Specifier::Short,
      Keyword::Int => Specifier::Int,
      Keyword::Long => Specifier::Long,
      Keyword::Float => Specifier::Float,
      Keyword::Double => Specifier::Double,
      Keyword::Signed => Specifier::Signed,
      Keyword::Unsigned => Specifier::Unsigned,
      Keyword::Bool => Specifier::Bool,
      _ => panic!("cannot convert {:?} to Specifier", kw),
    }
  }
}
impl From<&Literal> for Specifier {
  fn from(literal: &Literal) -> Self {
    match literal {
      Literal::Keyword(kw) => Specifier::from(kw),
      _ => panic!("cannot convert {:?} to Specifier", literal),
    }
  }
}
/// declaration-specifiers:
///    declaration-specifier attribute-specifier-sequenceopt (don't care)
///    declaration-specifier declaration-specifiers
/// declaration-specifier:
///    storage-class-specifier
///    type-specifier-qualifier
///    function-specifier
pub struct DeclSpecs {
  pub inline_hint: bool, // function-specifier: inline and _Noreturn
  pub storage_classes: Vec<Storage>,
  pub qualifiers: Vec<Qualifier>,
  pub specifiers: Vec<Specifier>,
}
pub struct Function {
  pub declspec: DeclSpecs,
  pub declarator: Declarator,
  pub body: Option<Compound>,
}
pub struct VarDef {
  pub declspec: DeclSpecs,
  pub declarator: Declarator,
  pub initializer: Option<Initializer>,
}

pub struct ArrayModifier {
  pub qualifiers: Vec<Qualifier>,
  pub is_static: bool,
  pub bound: ArrayBound,
}
pub enum ArrayBound {
  Constant(usize),
  Variable(Expression),
  Incomplete,
}
pub struct FunctionSignature {
  pub parameters: Vec<Parameter>,
  pub is_variadic: bool,
}
pub enum Initializer {
  Expression(Box<Expression>),
  List(Vec<InitializerListEntry>),
}
pub struct InitializerListEntry {
  pub designators: Vec<Designator>,
  pub value: Box<Initializer>,
}
pub enum Designator {
  Member(String),
  Index(Expression),
}
pub struct EnumSpecifier {
  pub name: Option<String>,
  pub enumerators: Vec<Enumerator>,
}
pub struct Enumerator {
  pub name: String,
  pub value: Option<Expression>,
}
impl Enumerator {
  pub fn new(name: String, value: Option<Expression>) -> Self {
    Self { name, value }
  }
}
impl EnumSpecifier {
  pub fn new(name: Option<String>, enumerators: Vec<Enumerator>) -> Self {
    Self { name, enumerators }
  }
}
impl Function {
  pub fn new(declspec: DeclSpecs, declarator: Declarator, body: Option<Compound>) -> Self {
    Self {
      declspec,
      declarator,
      body,
    }
  }
}
impl FunctionSignature {
  pub fn new(parameters: Vec<Parameter>, is_variadic: bool) -> Self {
    Self {
      parameters,
      is_variadic,
    }
  }
  pub fn default() -> Self {
    Self {
      parameters: Vec::new(),
      is_variadic: false,
    }
  }
}
impl Parameter {
  pub fn new(specifications: DeclSpecs, declarator: Declarator) -> Self {
    Self {
      specifications,
      declarator,
    }
  }
}
impl TranslationUnit {
  pub fn new() -> Self {
    Self {
      declarations: Vec::new(),
    }
  }
}

impl Keyword {
  pub fn to_type(&self) -> Option<Primitive> {
    Primitive::maybe_new(self.to_string())
  }
}
impl Declarator {
  pub fn new(name: Option<String>) -> Self {
    Self {
      name,
      modifiers: Vec::new(),
    }
  }
}
impl DeclSpecs {
  pub fn new() -> Self {
    Self {
      inline_hint: false,
      storage_classes: Vec::new(),
      qualifiers: Vec::new(),
      specifiers: Vec::new(),
    }
  }
}
impl VarDef {
  pub fn new(
    declspec: DeclSpecs,
    declarator: Declarator,
    initializer: Option<Initializer>,
  ) -> Self {
    Self {
      declspec,
      declarator,
      initializer,
    }
  }
}
mod fmt {
  use crate::parser::declaration::{
    DeclSpecs, Declaration, Function, FunctionSignature, Modifier, TranslationUnit, VarDef,
  };
  use ::std::fmt::{Debug, Display};

  impl Display for Declaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
        Declaration::Function(func) => <Function as Display>::fmt(func, f),
        Declaration::Variable(var) => <VarDef as Display>::fmt(var, f),
      }
    }
  }
  impl Debug for Declaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      <Self as Display>::fmt(self, f)
    }
  }

  impl Display for TranslationUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      self
        .declarations
        .iter()
        .try_for_each(|decl| write!(f, "{}\n", decl))
    }
  }
  impl Debug for TranslationUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      <Self as Display>::fmt(self, f)
    }
  }
  impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(
        f,
        "<{} {}: {} -> {}> {}",
        match &self.body {
          Some(_) => "function",
          None => "functiondecl",
        },
        match &self.declarator.name {
          Some(name) => name,
          None => "<anonymous>",
        },
        self
          .declarator
          .modifiers
          .iter()
          .map(|m| m.to_string())
          .collect::<Vec<_>>()
          .join(", "),
        self
          .declspec
          .specifiers
          .iter()
          .map(|s| s.to_string())
          .collect::<Vec<_>>()
          .join(" "),
        match &self.body {
          Some(block) => format!("{}", block),
          None => ";".to_string(),
        }
      )
    }
  }
  impl Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      <Self as Display>::fmt(self, f)
    }
  }

  impl Display for Modifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
        Modifier::Pointer(qualifiers) => {
          write!(
            f,
            "*{}",
            if qualifiers.is_empty() {
              "".to_string()
            } else {
              format!(
                " {}",
                qualifiers
                  .iter()
                  .map(|q| q.to_string())
                  .collect::<Vec<_>>()
                  .join(" ")
              )
            }
          )
        }
        Modifier::Array(_) => todo!(),
        Modifier::Function(function_signature) => {
          <FunctionSignature as Display>::fmt(function_signature, f)
        }
      }
    }
  }
  impl Debug for Modifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      <Self as Display>::fmt(self, f)
    }
  }

  impl Display for FunctionSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "(")?;
      for (i, param) in self.parameters.iter().enumerate() {
        if i > 0 {
          write!(f, ", ")?;
        }
        write!(
          f,
          "{}",
          param
            .specifications
            .specifiers
            .iter()
            .map(|m| m.to_string())
            .collect::<Vec<_>>()
            .join(" ")
        )?;
      }
      if self.is_variadic {
        if !self.parameters.is_empty() {
          write!(f, ", ")?;
        }
        write!(f, "...")?;
      }
      write!(f, ")")
    }
  }
  impl Debug for FunctionSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      <Self as Display>::fmt(self, f)
    }
  }
  impl Display for VarDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(
        f,
        "<variable {}>",
        match &self.declarator.name {
          Some(name) => name,
          None => "<anonymous>",
        },
      )
    }
  }
  impl Debug for VarDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      <Self as Display>::fmt(self, f)
    }
  }
  impl Display for DeclSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "<declaration specs>")
    }
  }
  impl Debug for DeclSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      <Self as Display>::fmt(self, f)
    }
  }
}
