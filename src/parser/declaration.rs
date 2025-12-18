use crate::common::keyword::Keyword;
use crate::parser::expression::Expression;
use crate::parser::statement::Statement;
use crate::parser::types::Primitive;

pub struct Program {
  pub declarations: Vec<Declaration>,
}
pub enum Declaration {
  Function(Function),
  Variable(VarDef),
}

pub enum Storage {
  Automatic,
  Static,
  Register,
  Extern,
  TypeDef, // ??? this counted as storage class?
}
pub enum Qualifier {
  Const,
  Volatile,
  Restrict,
}
pub enum Modifier {
  Pointer(Vec<Qualifier>),
  Array(ArrayModifier),
  Function(FunctionSignature),
}

// declarator contains the name
pub struct Declarator {
  pub name: String,
  pub modifiers: Vec<Modifier>,
}
pub struct Member {
  pub specifiers: Vec<Specifier>,
  pub qualifiers: Vec<Qualifier>,
  pub modifiers: Vec<Modifier>,
  pub declarator: Option<Declarator>,
  pub bit_width: Option<Expression>,
}
pub struct Parameter {
  pub specifications: Vec<DeclSpecs>,
  pub declarator: Option<Declarator>,
}
pub struct Struct {
  pub name: Option<String>,
  pub members: Vec<Member>,
}

pub enum Specifier {
  Void,
  Char,
  Short,
  Int,
  Long,
  Float,
  Double,
  Signed,
  Unsigned,
  Bool,
  Complex,
  Struct(Struct),
  Union(Struct),
  Enum(EnumSpecifier),
  TypedefName(String),
}
pub struct DeclSpecs {
  pub inline_hint: bool,
  pub storage_classes: Vec<Storage>,
  pub qualifiers: Vec<Qualifier>,
  pub specifiers: Vec<Specifier>,
}
pub struct Function {
  decl: FunctionDecl,
  body: Block,
}
pub struct VarDef {
  pub declspec: DeclSpecs,
  pub declarator: Declarator,
  pub initializer: Option<Initializer>,
}
pub struct FunctionDecl {
  pub declspec: DeclSpecs,
  pub name: String,
  pub modifiers: Vec<Modifier>,
}

pub struct Block {
  pub statements: Vec<Statement>,
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
  pub fn new(decl: FunctionDecl, body: Block) -> Self {
    Self { decl, body }
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

impl Keyword {
  pub fn to_type(&self) -> Option<Primitive> {
    Primitive::maybe_new(self.to_string())
  }
}
impl Declarator {
  pub fn new(name: String) -> Self {
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
impl FunctionDecl {
  pub fn new(declspec: DeclSpecs, name: String, modifiers: Vec<Modifier>) -> Self {
    Self {
      declspec,
      name,
      modifiers,
    }
  }
}
mod fmt {
  use crate::parser::declaration::{Block, DeclSpecs, Declaration, Function, Program, VarDef};
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

  impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      self
        .declarations
        .iter()
        .try_for_each(|decl| write!(f, "{}\n", decl))
    }
  }
  impl Debug for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      <Self as Display>::fmt(self, f)
    }
  }
  impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "<function {}>\n{}", self.decl.name, self.body)
    }
  }
  impl Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      <Self as Display>::fmt(self, f)
    }
  }
  impl Display for VarDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(
        f,
        "<variable {}>",
        if self.declarator.name.is_empty() {
          "<unnamed>"
        } else {
          &self.declarator.name
        }
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
  impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{{\n")?;
      for stmt in &self.statements {
        write!(f, "  {}\n", stmt)?;
      }
      write!(f, "}}")
    }
  }

  impl Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      <Self as Display>::fmt(self, f)
    }
  }
}
