use ::std::{
  fmt::{Debug, Display},
  str::FromStr,
};

use ::strum_macros::EnumString;

use crate::common::keyword::Keyword;
use crate::parser::statement::Statement;

pub struct Program {
  pub declarations: Vec<Declaration>,
}
impl Display for Program {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self
      .declarations
      .iter()
      .try_for_each(|decl| write!(f, "{:?}\n", decl))
  }
}
impl Debug for Program {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    <Self as Display>::fmt(self, f)
  }
}
// C's built-in types
#[derive(Debug, EnumString)]
pub enum Builtin {
  // assume 64bit
  #[strum(serialize = "char")]
  #[strum(serialize = "signed char")]
  Int8, // signed char/plain char
  #[strum(serialize = "short")]
  Int16, // short
  #[strum(serialize = "int")]
  #[strum(serialize = "long")]
  Int32, // int/long
  #[strum(serialize = "long long")]
  Int64, // long long
  #[strum(serialize = "unsigned char")]
  Uint8, // unsigned char
  #[strum(serialize = "unsigned short")]
  Uint16, // unsigned short
  #[strum(serialize = "unsigned int")]
  #[strum(serialize = "unsigned long")]
  Uint32, // unsigned int/unsigned long
  #[strum(serialize = "unsigned long long")]
  Uint64, // unsigned long long
  #[strum(serialize = "float")]
  Float32, // float
  #[strum(serialize = "double")]
  #[strum(serialize = "long double")]
  Float64, // double/long double
  #[strum(serialize = "bool")]
  #[strum(serialize = "_Bool")]
  Bool, // _Bool, or just bool
  #[strum(serialize = "void")]
  Void, // void
        // others: wchar_t, complex, etc. ignored for now
}
#[derive(Debug)]
pub struct UserDefined {
  name: String,
  fields: Vec<(String, Type)>,
}

pub enum Type {
  Builtin(Builtin),
  UserDefined(UserDefined),
  TypeDef(String),
}
impl Display for Type {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Type::Builtin(builtin) => write!(f, "{:?}", builtin),
      Type::UserDefined(user_defined) => write!(f, "{}", user_defined.name),
      Type::TypeDef(name) => write!(f, "{}", name),
    }
  }
}
impl Debug for Type {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    <Self as Display>::fmt(self, f)
  }
}
pub struct Function {
  name: String,
  parameters: Vec<(String, Type)>,
  body: Block,
  return_type: Type,
}
impl Display for Function {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Function {}(", self.name)?;
    for (i, (param_name, param_type)) in self.parameters.iter().enumerate() {
      write!(f, "{}: {}", param_name, param_type)?;
      if i != self.parameters.len() - 1 {
        write!(f, ", ")?;
      }
    }
    write!(f, ") -> {:?} ", self.return_type)?;
    write!(f, "{{\n")?;
    for stmt in &self.body.statements {
      write!(f, "  {:?}\n", stmt)?;
    }
    write!(f, "}}")
  }
}
impl Debug for Function {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    <Self as Display>::fmt(self, f)
  }
}
pub struct Variable {}
impl Display for Variable {
  fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    Ok(()) // ignore for now
  }
}
impl Debug for Variable {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    <Self as Display>::fmt(self, f)
  }
}
pub enum Declaration {
  Function(Function),
  Variable(Variable),
}
impl Display for Declaration {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Declaration::Function(func) => <Function as Display>::fmt(func, f),
      Declaration::Variable(var) => <Variable as Display>::fmt(var, f),
    }
  }
}
impl Debug for Declaration {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    <Self as Display>::fmt(self, f)
  }
}

pub struct Block {
  pub statements: Vec<Statement>,
}

impl Display for Block {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{{\n")?;
    for stmt in &self.statements {
      write!(f, "  {:?}\n", stmt)?;
    }
    write!(f, "}}")
  }
}

impl Debug for Block {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    <Self as Display>::fmt(self, f)
  }
}

impl Block {
  pub fn new() -> Self {
    Self {
      statements: Vec::new(),
    }
  }
}

impl Program {
  pub fn new() -> Self {
    Self {
      declarations: Vec::new(),
    }
  }
}

impl Function {
  pub fn new(
    name: String,
    parameters: Vec<(String, Type)>,
    body: Block,
    return_type: Type,
  ) -> Self {
    Self {
      name,
      parameters,
      body,
      return_type,
    }
  }
}

impl Type {}

impl Builtin {
  pub fn new(str: String) -> Self {
    Self::maybe_new(str).unwrap()
  }
  pub fn maybe_new(str: String) -> Option<Self> {
    Builtin::from_str(&str).ok()
  }
  pub fn as_type(self) -> Type {
    Type::Builtin(self)
  }
}

impl Keyword {
  pub fn to_type(&self) -> Option<Builtin> {
    Builtin::maybe_new(self.to_string())
  }
}
