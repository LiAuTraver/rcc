use ::rcc_utils::SmallString;

use crate::{
  common::SourceSpan,
  parser::declaration::{DeclSpecs, Declarator},
  type_alias_expr,
};

#[derive(Debug)]
pub enum Expression {
  Empty(Empty), // no-op for error recovery; for empty expr should use Option<Expression> instead
  Constant(Constant),
  Unary(Unary),
  Binary(Binary),
  Variable(Variable),
  Call(Call),
  Paren(Paren),
  MemberAccess(MemberAccess),
  Ternary(Ternary),
  SizeOf(SizeOf),
  CStyleCast(CStyleCast),           // (int)x
  ArraySubscript(ArraySubscript),   // arr[i]
  CompoundLiteral(CompoundLiteral), // (struct Point){.x=1, .y=2}
}
type_alias_expr! {Expression, UnprocessedType, Variable}
::rcc_utils::interconvert!(Variable, Expression);
::rcc_utils::interconvert!(Constant, Expression);
::rcc_utils::interconvert!(Unary, Expression);
::rcc_utils::interconvert!(Binary, Expression);
::rcc_utils::interconvert!(Call, Expression);
::rcc_utils::interconvert!(Paren, Expression);
::rcc_utils::interconvert!(MemberAccess, Expression);
::rcc_utils::interconvert!(Ternary, Expression);
::rcc_utils::interconvert!(SizeOf, Expression);
::rcc_utils::interconvert!(CStyleCast, Expression);
::rcc_utils::interconvert!(ArraySubscript, Expression);
::rcc_utils::interconvert!(CompoundLiteral, Expression);
impl ::std::default::Default for Expression {
  #[inline(always)]
  fn default() -> Self {
    Expression::Empty(Empty::default())
  }
}

impl Variable {
  pub fn new(name: SmallString, span: SourceSpan) -> Self {
    Self { name, span }
  }
}
#[derive(Debug)]
pub struct UnprocessedType {
  pub declspecs: DeclSpecs,
  pub declarator: Declarator,
}
impl UnprocessedType {
  pub fn new(declspecs: DeclSpecs, declarator: Declarator) -> Self {
    Self {
      declspecs,
      declarator,
    }
  }
}
#[derive(Debug)]
pub struct Variable {
  pub name: SmallString,
  pub span: SourceSpan,
}

mod fmt {
  use ::rcc_utils::static_dispatch;
  use ::std::fmt::Display;

  use super::*;

  impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      static_dispatch!(
        self.fmt(f),
        Empty Constant Unary Binary Variable Call Paren MemberAccess Ternary SizeOf CStyleCast ArraySubscript CompoundLiteral
      )
    }
  }
  impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{}", self.name)
    }
  }
  impl Display for UnprocessedType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{} {}", self.declspecs, self.declarator)
    }
  }
}
