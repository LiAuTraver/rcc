//! likely a sophisticated version of the Two-Level Types described in
//! [this article](https://blog.ezyang.com/2013/05/the-ast-typing-problem/),
//! I probably used the Parametric Polymorphism to "tie the knot" of recursion.

use ::rcc_utils::IntoWith;

use crate::{
  common::{Operator, SourceSpan, StrRef},
  types::Constant,
};
macro_rules! type_alias_expr {
  ($exprty:ty, $typety:ty, $($extra:ident)*) => {
    type_alias_expr!(@impl $exprty, $typety, $($extra)*: []);
  };
  ($exprty:ty, $typety:ty, $($extra:ident $(<$lts:lifetime>)*)*) =>{
    type_alias_expr!(@impl $exprty, $typety, $($extra $(<$lts>)*)*);
  };
  (@impl $exprty:ty, $typety:ty, $($extra:ident $(<$lts:lifetime>)?)*) => {
    #[derive(Debug)]
    pub enum RawExpr<'context> {
      // no-op for error recovery; for empty expr should use Option<ExprTy> instead
      Empty(Empty),
      Constant(Constant<'context>),
      Unary(Unary<'context>),
      Binary(Binary<'context>),
      Call(Call<'context>),
      Paren(Paren<'context>),
      MemberAccess(MemberAccess<'context>),
      Ternary(Ternary<'context>),
      SizeOf(SizeOf<'context>),
      CStyleCast(CStyleCast<'context>),
      ArraySubscript(ArraySubscript<'context>),
      CompoundLiteral(CompoundLiteral),
      $(
        // Generate a variant for each extra type
        $extra($extra $(<$lts>)?),
      )*
    }
    pub type Empty = $crate::blueprints::Placeholder;
    /// exists to avoid name clash with `Constant` in this module; this is a design mistake
    pub type ConstantLiteral<'context> = $crate::types::Constant<'context>;
    /// type or expression
    pub type SizeOfKind<'context> = $crate::blueprints::RawSizeOfKind<$exprty, $typety>;
    /// unary kind
    pub type UnaryKind = $crate::blueprints::RawUnaryKind;
    pub type Constant<'context> = $crate::blueprints::RawConstant<'context>;
    pub type Unary<'context> = $crate::blueprints::RawUnary<$exprty>;
    pub type Binary<'context> = $crate::blueprints::RawBinary<$exprty>;
    pub type Call<'context> = $crate::blueprints::RawCall<$exprty>;
    pub type Paren<'context> = $crate::blueprints::RawParen<$exprty>;
    pub type MemberAccess<'context> = $crate::blueprints::RawMemberAccess<'context, $exprty>;
    pub type Ternary<'context> = $crate::blueprints::RawTernary<$exprty>;
    pub type SizeOf<'context> = $crate::blueprints::RawSizeOf<$exprty, $typety>;
    pub type CStyleCast<'context> = $crate::blueprints::RawCStyleCast<$exprty>;
    pub type ArraySubscript<'context> = $crate::blueprints::RawArraySubscript<$exprty>;
    pub type CompoundLiteral = $crate::blueprints::RawCompoundLiteral;

    mod fmtrawexpr {
      use super::*;
      impl<'context> ::std::fmt::Display for RawExpr<'context> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
          ::rcc_utils::static_dispatch!(
            self.fmt(f),
            Empty Constant Unary Binary Variable Call Paren MemberAccess Ternary SizeOf CStyleCast ArraySubscript CompoundLiteral
            $($extra)*
          )
        }
      }
    }
    mod cvtrawexpr {
      use super::*;

      ::rcc_utils::interconvert!(Empty, RawExpr<'context>);
      ::rcc_utils::interconvert!(Constant, RawExpr,'context);
      ::rcc_utils::interconvert!(Unary, RawExpr, 'context);
      ::rcc_utils::interconvert!(Binary, RawExpr, 'context);
      ::rcc_utils::interconvert!(Call, RawExpr, 'context);
      ::rcc_utils::interconvert!(Paren, RawExpr, 'context);
      ::rcc_utils::interconvert!(MemberAccess, RawExpr, 'context);
      ::rcc_utils::interconvert!(Ternary, RawExpr, 'context);
      ::rcc_utils::interconvert!(SizeOf, RawExpr, 'context);
      ::rcc_utils::interconvert!(CStyleCast, RawExpr, 'context);
      ::rcc_utils::interconvert!(ArraySubscript, RawExpr, 'context);
      ::rcc_utils::interconvert!(CompoundLiteral, RawExpr<'context>);
      $(
        ::rcc_utils::interconvert!($extra, RawExpr, $($lts)?);
      )*

      ::rcc_utils::make_trio_for!(Empty, RawExpr<'context>);
      ::rcc_utils::make_trio_for!(Constant, RawExpr, 'context);
      ::rcc_utils::make_trio_for!(Unary, RawExpr, 'context);
      ::rcc_utils::make_trio_for!(Binary, RawExpr, 'context);
      ::rcc_utils::make_trio_for!(Call, RawExpr, 'context);
      ::rcc_utils::make_trio_for!(Paren, RawExpr, 'context);
      ::rcc_utils::make_trio_for!(MemberAccess, RawExpr, 'context);
      ::rcc_utils::make_trio_for!(Ternary, RawExpr, 'context);
      ::rcc_utils::make_trio_for!(SizeOf, RawExpr, 'context);
      ::rcc_utils::make_trio_for!(CStyleCast, RawExpr, 'context);
      ::rcc_utils::make_trio_for!(ArraySubscript, RawExpr, 'context);
      ::rcc_utils::make_trio_for!(CompoundLiteral, RawExpr<'context>);
      $(
        ::rcc_utils::make_trio_for!($extra, RawExpr, $($lts)?);
      )*

      impl<'context> ::rcc_utils::IntoWith<SourceSpan, RawExpr<'context>> for ConstantLiteral<'context> {
        fn into_with(self, span: SourceSpan) -> RawExpr<'context> {
          RawExpr::Constant(self.into_with(span))
        }
      }

      impl<'context> ::rcc_utils::IntoWith<SourceSpan, RawExpr<'context>> for SizeOfKind<'context> {
        fn into_with(self, span: SourceSpan) -> RawExpr<'context> {
          RawExpr::SizeOf(self.into_with(span))
        }
      }
    }

    mod getspan {
      use super::*;
      use $crate::common::SourceSpan;
      impl<'context> RawExpr<'context> {
        pub fn span(&self) -> SourceSpan {
          match self {
            RawExpr::Empty(_) => SourceSpan::default(),
            RawExpr::Constant(c) => c.span,
            RawExpr::Unary(u) => u.span,
            RawExpr::Binary(b) => b.span,
            RawExpr::Call(call) => call.span,
            RawExpr::Paren(p) => p.span,
            RawExpr::MemberAccess(ma) => ma.span,
            RawExpr::Ternary(t) => t.span,
            RawExpr::SizeOf(sizeof) => sizeof.span,
            RawExpr::CStyleCast(cast) => cast.span,
            RawExpr::ArraySubscript(arrsub) => arrsub.span,
            RawExpr::CompoundLiteral(cl) => cl.span,
            $(
              RawExpr::$extra(inner) => inner.span,
            )*
          }
        }
      }
    }

    ::rcc_utils::static_assert!(
      ::std::mem::size_of::<RawExpr>() <= 64,
      "RawExpr size exceeds 64 bytes",
    );

  };
}
pub(in super::super) use type_alias_expr;

#[derive(Debug)]
pub struct RawConstant<'context> {
  pub value: Constant<'context>,
  pub span: SourceSpan,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::Display)]
pub enum RawUnaryKind {
  #[strum(to_string = "prefix")]
  Prefix,
  #[strum(serialize = "postfix")]
  Postfix,
}
#[derive(Debug)]
pub struct RawUnary<ExprTy> {
  pub operator: Operator,
  pub operand: Box<ExprTy>,
  pub kind: RawUnaryKind,
  pub span: SourceSpan,
}
#[derive(Debug)]
pub struct RawBinary<ExprTy> {
  pub operator: Operator,
  pub left: Box<ExprTy>,
  pub right: Box<ExprTy>,
  pub span: SourceSpan,
}
#[derive(Debug)]
pub struct RawCall<ExprTy> {
  pub callee: Box<ExprTy>,
  pub arguments: Vec<ExprTy>,
  pub span: SourceSpan,
}
#[derive(Debug)]
pub struct RawParen<ExprTy> {
  pub expr: Box<ExprTy>,
  pub span: SourceSpan,
}
#[derive(Debug)]
pub struct RawMemberAccess<'context, ExprTy> {
  pub object: Box<ExprTy>,
  pub member: StrRef<'context>,
  pub span: SourceSpan,
}
#[derive(Debug)]
pub struct RawTernary<ExprTy> {
  pub condition: Box<ExprTy>,
  pub then_expr: Box<ExprTy>,
  pub else_expr: Box<ExprTy>,
  pub span: SourceSpan,
}
#[derive(Debug)]
pub enum RawSizeOfKind<ExprTy, TypeTy> {
  Type(Box<TypeTy>), // ignore for now
  Expression(Box<ExprTy>),
}

#[derive(Debug)]
pub struct RawSizeOf<ExprTy, TypeTy> {
  pub sizeof: RawSizeOfKind<ExprTy, TypeTy>,
  pub span: SourceSpan,
}

#[derive(Debug)]
pub struct RawCStyleCast<ExprTy> {
  // pub target_type: Box<QualifiedType>,
  pub expr: Box<ExprTy>,
  pub span: SourceSpan,
}
#[derive(Debug)]
pub struct RawArraySubscript<ExprTy> {
  pub array: Box<ExprTy>,
  pub index: Box<ExprTy>,
  pub span: SourceSpan,
}
#[derive(Debug)]
pub struct RawCompoundLiteral {
  // pub target_type: Box<QualifiedType>,
  // pub initializer: Initializer,
  pub span: SourceSpan,
}

impl<'context> RawConstant<'context> {
  pub fn new(constant: Constant<'context>, span: SourceSpan) -> Self {
    Self {
      value: constant,
      span,
    }
  }
}

impl<'context> ::std::ops::Deref for RawConstant<'context> {
  type Target = Constant<'context>;

  fn deref(&self) -> &Self::Target {
    &self.value
  }
}

impl<'context> IntoWith<SourceSpan, RawConstant<'context>>
  for Constant<'context>
{
  fn into_with(self, span: SourceSpan) -> RawConstant<'context> {
    RawConstant::new(self, span)
  }
}

impl<ExprTy> RawUnary<ExprTy> {
  pub fn new(
    operator: Operator,
    operand: ExprTy,
    kind: RawUnaryKind,
    span: SourceSpan,
  ) -> Self {
    debug_assert!(operator.unary(), "not a unary operator! got {:?}", operator);
    Self {
      operator,
      operand: operand.into(),
      kind,
      span,
    }
  }

  #[inline(always)]
  pub fn prefix(operator: Operator, operand: ExprTy, span: SourceSpan) -> Self {
    debug_assert!(operator.unary(), "not a unary operator! got {:?}", operator);
    Self::new(operator, operand, RawUnaryKind::Prefix, span)
  }

  #[inline(always)]
  pub fn postfix(
    operator: Operator,
    operand: ExprTy,
    span: SourceSpan,
  ) -> Self {
    debug_assert!(operator.unary(), "not a unary operator! got {:?}", operator);
    Self::new(operator, operand, RawUnaryKind::Postfix, span)
  }
}

impl<ExprTy> RawBinary<ExprTy> {
  pub fn from_operator(
    operator: Operator,
    left: ExprTy,
    right: ExprTy,
    span: SourceSpan,
  ) -> Option<Self> {
    match operator.binary() {
      true => Some(Self {
        operator,
        left: left.into(),
        right: right.into(),
        span,
      }),
      false => None,
    }
  }

  pub fn from_operator_unchecked(
    operator: Operator,
    left: ExprTy,
    right: ExprTy,
    span: SourceSpan,
  ) -> Self {
    debug_assert!(operator.binary());
    Self {
      operator,
      left: left.into(),
      right: right.into(),
      span,
    }
  }

  #[inline]
  pub fn new(
    operator: Operator,
    left: ExprTy,
    right: ExprTy,
    span: SourceSpan,
  ) -> Self {
    Self::from_operator(operator, left, right, span)
      .expect("not a binary operator! use `from_operator` instead")
  }
}
impl<ExprTy> RawTernary<ExprTy> {
  pub fn new(
    condition: ExprTy,
    then_expr: ExprTy,
    else_expr: ExprTy,
    span: SourceSpan,
  ) -> Self {
    Self {
      condition: condition.into(),
      then_expr: then_expr.into(),
      else_expr: else_expr.into(),
      span,
    }
  }
}

impl<ExprTy> RawArraySubscript<ExprTy> {
  pub fn new(array: ExprTy, index: ExprTy, span: SourceSpan) -> Self {
    Self {
      array: array.into(),
      index: index.into(),
      span,
    }
  }
}
impl<ExprTy, TypeTy> RawSizeOf<ExprTy, TypeTy> {
  pub fn new(sizeof: RawSizeOfKind<ExprTy, TypeTy>, span: SourceSpan) -> Self {
    Self { sizeof, span }
  }
}

impl<ExprTy, TypeTy> IntoWith<SourceSpan, RawSizeOf<ExprTy, TypeTy>>
  for RawSizeOfKind<ExprTy, TypeTy>
{
  fn into_with(self, span: SourceSpan) -> RawSizeOf<ExprTy, TypeTy> {
    RawSizeOf::new(self, span)
  }
}
impl<ExprTy> RawCall<ExprTy> {
  pub fn new(callee: ExprTy, arguments: Vec<ExprTy>, span: SourceSpan) -> Self {
    Self {
      callee: callee.into(),
      arguments,
      span,
    }
  }
}
impl<ExprTy> RawParen<ExprTy> {
  pub fn new(expr: ExprTy, span: SourceSpan) -> Self {
    Self {
      expr: expr.into(),
      span,
    }
  }
}
mod fmt {
  use ::std::fmt::Display;

  use super::*;

  impl<'context> Display for RawConstant<'context> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{}", self.value)
    }
  }

  impl<ExprTy: Display> Display for RawCall<ExprTy> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{}(", self.callee)?;
      for (i, arg) in self.arguments.iter().enumerate() {
        write!(f, "{}", arg)?;
        if i != self.arguments.len() - 1 {
          write!(f, ", ")?;
        }
      }
      write!(f, ")")
    }
  }
  impl<ExprTy: Display> Display for RawUnary<ExprTy> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      if matches!(self.operator, Operator::PlusPlus | Operator::MinusMinus) {
        write!(f, "({} {}{})", self.operand, self.kind, self.operator)
      } else {
        write!(f, "({} {})", self.operand, self.operator)
      }
    }
  }
  impl<ExprTy: Display> Display for RawBinary<ExprTy> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "({} {} {})", self.left, self.right, self.operator)
    }
  }
  impl<ExprTy: Display> Display for RawTernary<ExprTy> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(
        f,
        "({} ? {} : {})",
        self.condition, self.then_expr, self.else_expr
      )
    }
  }
  impl<ExprTy: Display> Display for RawParen<ExprTy> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "({})", self.expr)
    }
  }
  impl<ExprTy: Display, TypeTy: Display> Display for RawSizeOf<ExprTy, TypeTy> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{}", self.sizeof)
    }
  }
  impl<ExprTy: Display, TypeTy: Display> Display
    for RawSizeOfKind<ExprTy, TypeTy>
  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
        RawSizeOfKind::Type(typ) => write!(f, "sizeof({})", typ),
        RawSizeOfKind::Expression(expr) => write!(f, "sizeof({})", expr),
      }
    }
  }
  // noop display impl for the rest
  impl<ExprTy: Display> Display for RawArraySubscript<ExprTy> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "(({})[{}])", self.array, self.index)
    }
  }
  impl Display for RawCompoundLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "<compound literal - not implemented>")
    }
  }
  impl<ExprTy: Display> Display for RawCStyleCast<ExprTy> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "<C-style cast - not implemented>")
    }
  }
  impl<'context, ExprTy: Display> Display for RawMemberAccess<'context, ExprTy> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "({}.{})", self.object, self.member)
    }
  }
}
