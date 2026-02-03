use ::rc_utils::{
  IntoWith, contract_assert, contract_violation, static_dispatch,
};

use super::expression::{
  ArraySubscript, Assignment, Binary, CStyleCast, Call, CompoundLiteral,
  Constant, ConstantLiteral as CL, Empty, Expression, ImplicitCast,
  MemberAccess, Paren, RawExpr, SizeOf, SizeOfKind, Ternary, Unary,
  ValueCategory, Variable,
};
use crate::{
  common::{Floating, Integral, Operator, SourceSpan},
  diagnosis::{DiagData::*, Diagnosis},
  types::{CastType, Compatibility, QualifiedType, Type, TypeInfo},
};

#[derive(Debug)]
pub enum FoldingResult<T> {
  Success(T),
  Failure(T),
}
impl<T> ::std::ops::FromResidual for FoldingResult<T> {
  fn from_residual(residual: <Self as ::std::ops::Try>::Residual) -> Self {
    residual
  }
}

impl<T> ::std::ops::Try for FoldingResult<T> {
  type Output = T;
  type Residual = FoldingResult<T>;

  fn from_output(output: Self::Output) -> Self {
    Self::Success(output)
  }

  fn branch(self) -> ::std::ops::ControlFlow<Self::Residual, Self::Output> {
    match self {
      Self::Success(v) => ::std::ops::ControlFlow::Continue(v),
      _ => ::std::ops::ControlFlow::Break(self),
    }
  }
}

impl<T> FoldingResult<T> {
  fn map<U>(self, f: impl FnOnce(T) -> U) -> FoldingResult<U> {
    match self {
      Self::Success(v) => FoldingResult::Success(f(v)),
      Self::Failure(v) => FoldingResult::Failure(f(v)),
    }
  }

  pub fn unwrap(self) -> T {
    match self {
      Self::Failure(v) | Self::Success(v) => v,
    }
  }

  pub fn transform<U>(self, f: impl FnOnce(T) -> U) -> U {
    match self {
      Self::Success(v) | Self::Failure(v) => f(v),
    }
  }
}

/// Folding trait for constant expression evaluation
pub trait Folding {
  /// This serves as a never-fail folding mechanism,
  /// all errors and warnings shall be handled via `diag` parameter.
  /// [`Operational`](crate::diagnosis::Operational) is recommended.
  /// If folding is not possible, return self unchanged.
  /// So it may end up being a no-op, partial-fold, or full-fold.
  ///
  /// If [`Diagnosis`] is not required, use [`NoOp`](crate::diagnosis::NoOp) as the dummy parameter.
  #[must_use]
  fn fold(
    self,
    target_type: QualifiedType,
    value_category: ValueCategory,
    diag: &impl Diagnosis,
  ) -> FoldingResult<Expression>;
}

use FoldingResult::{Failure, Success};

impl Expression {
  #[inline(always)]
  pub(super) fn fold(self, diag: &impl Diagnosis) -> FoldingResult<Expression> {
    let (raw_expr, expr_type, value_category) = self.destructure();
    raw_expr.fold(expr_type, value_category, diag)
  }
}
impl Folding for Empty {
  #[inline(always)]
  fn fold(
    self,
    target_type: QualifiedType,
    value_category: ValueCategory,
    _diag: &impl Diagnosis,
  ) -> FoldingResult<Expression> {
    Failure(Expression::new(self.into(), target_type, value_category))
  }
}

impl Folding for Call {
  #[inline(always)]
  fn fold(
    self,
    target_type: QualifiedType,
    value_category: ValueCategory,
    _diag: &impl Diagnosis,
  ) -> FoldingResult<Expression> {
    Failure(Expression::new(self.into(), target_type, value_category))
  }
}

impl Folding for MemberAccess {
  fn fold(
    self,
    _target_type: QualifiedType,
    _value_category: ValueCategory,
    _diag: &impl Diagnosis,
  ) -> FoldingResult<Expression> {
    todo!()
  }
}
impl Folding for CStyleCast {
  fn fold(
    self,
    _target_type: QualifiedType,
    _value_category: ValueCategory,
    _diag: &impl Diagnosis,
  ) -> FoldingResult<Expression> {
    todo!()
  }
}

impl Folding for ArraySubscript {
  fn fold(
    self,
    _target_type: QualifiedType,
    _value_category: ValueCategory,
    _diag: &impl Diagnosis,
  ) -> FoldingResult<Expression> {
    todo!()
  }
}

impl Folding for CompoundLiteral {
  fn fold(
    self,
    _target_type: QualifiedType,
    _value_category: ValueCategory,
    _diag: &impl Diagnosis,
  ) -> FoldingResult<Expression> {
    todo!()
  }
}

impl Folding for Assignment {
  /// assignment expr is not considered constant expr in C, but in C++ it is.
  #[inline(always)]
  fn fold(
    self,
    target_type: QualifiedType,
    value_category: ValueCategory,
    _diag: &impl Diagnosis,
  ) -> FoldingResult<Expression> {
    Failure(Expression::new(self.into(), target_type, value_category))
  }
}
impl Folding for RawExpr {
  fn fold(
    self,
    target_type: QualifiedType,
    value_category: ValueCategory,
    diag: &impl Diagnosis,
  ) -> FoldingResult<Expression> {
    static_dispatch!(
      self.fold(target_type, value_category, diag),
      Empty Constant Unary Binary Call Paren MemberAccess Ternary SizeOf CStyleCast ArraySubscript CompoundLiteral Variable ImplicitCast Assignment
    )
  }
}
impl Folding for Constant {
  #[inline(always)]
  fn fold(
    self,
    target_type: QualifiedType,
    value_category: ValueCategory,
    _diag: &impl Diagnosis,
  ) -> FoldingResult<Expression> {
    Success(Expression::new(self.into(), target_type, value_category))
  }
}

impl Folding for Unary {
  fn fold(
    self,
    target_type: QualifiedType,
    value_category: ValueCategory,
    diag: &impl Diagnosis,
  ) -> FoldingResult<Expression> {
    debug_assert!(
      self.operator.unary(),
      "not an unary operator! should not happen!"
    );

    let folded_operand = self.operand.fold(diag)?;

    contract_assert!(
      folded_operand.raw_expr().is_constant(),
      "only implemented for constant var of constant eval"
    );

    let raw_constant = match self.operator {
      // unary `+` is no-op for arithmetic types
      Operator::Plus => return Success(folded_operand),
      // this happens after promotion, so no need to worry about smaller types
      Operator::Minus => todo!(),
      // match &folded_operand.raw_expr().as_constant_unchecked().constant {
      //   // as-is!
      //   _ => contract_violation!(
      //     "the unary '-' applied to non-numeric constant or types that should be promoted: {:?}",
      //     folded_operand.raw_expr().as_constant_unchecked().constant
      //   ),
      // },
      Operator::Not => if folded_operand
        .raw_expr()
        .as_constant_unchecked()
        .constant
        .is_zero()
      {
        Success(Integral::from_bool(true))
      } else {
        Success(Integral::from_bool(false))
      }
      .map(|c: Integral| c.into()),
      _ => todo!(),
    };
    raw_constant.map(|constant: CL| {
      Expression::new(
        constant.into_with(self.span),
        target_type,
        value_category,
      )
    })
  }
}

impl Folding for Binary {
  fn fold(
    self,
    target_type: QualifiedType,
    value_category: ValueCategory,
    diag: &impl Diagnosis,
  ) -> FoldingResult<Expression> {
    debug_assert!(
      self.operator.binary(),
      "not a binary operator! should not happen!"
    );
    let fl = self.left.fold(diag);
    let fr = self.right.fold(diag);
    let (folded_lhs, folded_rhs) =
      if matches!(fl, Success(_)) && matches!(fr, Success(_)) {
        (fl.unwrap(), fr.unwrap())
      } else {
        return Failure(Expression::new(
          Self {
            left: fl.unwrap().into(),
            right: fr.unwrap().into(),
            ..self
          }
          .into(),
          target_type,
          value_category,
        ));
      };
    assert!(
      folded_lhs.raw_expr().is_constant()
        && folded_rhs.raw_expr().is_constant(),
      "only implemented for constant var of constant eval"
    );
    assert!(
      folded_lhs.qualified_type() == folded_rhs.qualified_type(),
      "type checker makes sure both sides have the same type via `ImplicitCast`!"
    );
    let (lhs_expr, lhs_type, lhs_value_category) = folded_lhs.destructure();
    let (rhs_expr, rhs_type, rhs_value_category) = folded_rhs.destructure();

    assert!(
      lhs_type == rhs_type,
      "type checker ensures both sides have the same types!"
    );

    assert!(
      lhs_value_category == ValueCategory::RValue
        && rhs_value_category == ValueCategory::RValue,
      "type checker ensures both sides are rvalues!"
    );

    let lhs = lhs_expr
      .into_constant()
      .expect("shall be constant")
      .constant;
    let rhs = rhs_expr
      .into_constant()
      .expect("shall be constant")
      .constant;
    match (lhs, rhs) {
          (
            crate::types::Constant::Integral(lhs),
            crate::types::Constant::Integral(rhs),
          ) => Integral::handle_binary_op(self.operator, lhs, rhs, self.span, diag).map(Into::into),
          (
            crate::types::Constant::Floating(lhs),
            crate::types::Constant::Floating(rhs),
          ) => Floating::handle_binary_op(self.operator, lhs, rhs, self.span, diag).map(Into::into),
          (
            crate::types::Constant::String(_),
            crate::types::Constant::String(_),
          ) => contract_violation!("can we reach here?"),
          (
            crate::types::Constant::Nullptr(_),
            crate::types::Constant::Nullptr(_),
          ) => contract_violation!("can we reach here?"),
          _ => contract_violation!(
            "type checker ensures both sides have the same types! or unimplemented type"
          ),
        }
        .map(|constant:CL| {
          Expression::new(
        constant.into_with(self.span),
        target_type,
        value_category,
      )
    })
  }
}
impl Folding for Ternary {
  fn fold(
    self,
    target_type: QualifiedType,
    value_category: ValueCategory,
    diag: &impl Diagnosis,
  ) -> FoldingResult<Expression> {
    debug_assert!(
      self
        .then_expr
        .qualified_type()
        .compatible_with(self.else_expr.qualified_type()),
      "type checker ensures both branches have compatible types!"
    );
    let fc = self.condition.fold(diag);
    let ft = self.then_expr.fold(diag);
    let fe = self.else_expr.fold(diag);

    let is_success =
      matches!((&fc, &ft, &fe), (Success(_), Success(_), Success(_)));

    let expr = Expression::new(
      Self {
        condition: fc.unwrap().into(),
        then_expr: ft.unwrap().into(),
        else_expr: fe.unwrap().into(),
        ..self
      }
      .into(),
      target_type,
      value_category,
    );

    match is_success {
      true => Success(expr),
      false => Failure(expr),
    }
  }
}

impl Folding for SizeOf {
  fn fold(
    self,
    target_type: QualifiedType,
    value_category: ValueCategory,
    diag: &impl Diagnosis,
  ) -> FoldingResult<Expression> {
    match self.sizeof {
      SizeOfKind::Type(qualified_type) => if qualified_type.size() > 0 {
        Success(Integral::from_ulong_long(qualified_type.size() as u64))
      } else {
        Failure(Integral::from_ulong_long(0))
      }
      .map(Integral::into)
      .map(|constant: CL| {
        Expression::new(
          constant.into_with(self.span),
          target_type,
          value_category,
        )
      }),
      SizeOfKind::Expression(expr) => expr.fold(diag),
    }
  }
}

impl Folding for Variable {
  fn fold(
    self,
    target_type: QualifiedType,
    value_category: ValueCategory,
    diag: &impl Diagnosis,
  ) -> FoldingResult<Expression> {
    if self.name.borrow().is_constexpr() {
      diag.add_error(
        UnsupportedFeature("constexpr variable not implemented".to_string()),
        self.span,
      );
      Failure(Expression::new(self.into(), target_type, value_category))
    } else {
      Failure(Expression::new(self.into(), target_type, value_category))
    }
  }
}

impl Folding for Paren {
  fn fold(
    self,
    _target_type: QualifiedType,
    _value_category: ValueCategory,
    diag: &impl Diagnosis,
  ) -> FoldingResult<Expression> {
    self.expr.fold(diag).map(|expr| expr)
  }
}

impl Folding for ImplicitCast {
  fn fold(
    self,
    target_type: QualifiedType,
    value_category: ValueCategory,
    diag: &impl Diagnosis,
  ) -> FoldingResult<Expression> {
    let folded_expr = self.expr.fold(diag)?;
    let (raw_expr, expr_type, _value_category) = folded_expr.destructure();

    use CastType::*;
    match self.cast_type {
      Noop | ToVoid | LValueToRValue | BitCast => Success(raw_expr),
      ArrayToPointerDecay => todo!("address constant"),
      FunctionToPointerDecay => todo!("address constant"),
      NullptrToPointer => match target_type.unqualified_type() {
        Type::Pointer(_) => todo!(),
        _ => contract_violation!("unreachable"),
      },
      IntegralCast => match raw_expr {
        RawExpr::Constant(c) => {
          let target_primitive =
            target_type.unqualified_type().as_primitive_unchecked();
          let expr_primitive =
            expr_type.unqualified_type().as_primitive_unchecked();
          if target_primitive.integer_rank() < expr_primitive.integer_rank() {
            diag.add_warning(
              CastDown(expr_type.clone(), target_type.clone()),
              self.span,
            )
          }
          todo!("{c}")
        },
        _ => contract_violation!("unreachable"),
      },
      // integral are promoted previously.
      IntegralToFloating => {
        todo!()
      },
      IntegralToBoolean | FloatingToBoolean => Success(
        raw_expr
          .into_constant_unchecked()
          .to_boolean()
          .into_with(self.span),
      ),
      FloatingCast => todo!(),
      FloatingToIntegral => todo!(),
      IntegralToPointer => contract_violation!(
        "no such implicit cast in C -- only for explicit casts!"
      ),
      PointerToIntegral => Failure(raw_expr),
      PointerToBoolean => Failure(raw_expr),
      NullptrToIntegral => match target_type.unqualified_type() {
        Type::Primitive(p) =>
          Success(Integral::new(p.size(), 0, p.is_signed().into()).into()),
        _ => contract_violation!("unreachable"),
      }
      .map(|c: CL| c.into_with(self.span)),
      NullptrToBoolean => Success(Integral::from_bool(false).into())
        .map(|c: CL| c.into_with(self.span)),
    }
    .map(|raw_expr| Expression::new(raw_expr, target_type, value_category))
  }
}
impl Integral {
  pub fn handle_binary_op(
    op: Operator,
    lhs: Self,
    rhs: Self,
    span: SourceSpan,
    diag: &impl Diagnosis,
  ) -> FoldingResult<Self> {
    debug_assert!(op.binary());
    debug_assert_eq!(lhs.width(), rhs.width());
    debug_assert_eq!(lhs.signedness(), rhs.signedness());

    macro_rules! arith {
      ($func:ident, $op:expr) => {{
        let (result, overflow) = lhs.$func(rhs);
        if overflow {
          diag.add_warning(
            ArithmeticBinOpOverflow((lhs.into(), rhs.into(), $op).into()),
            span,
          );
        }
        Success(result)
      }};
    }
    macro_rules! div_zero {
      ($func:ident) => {{
        match lhs.$func(rhs) {
          Some(result) => Success(result),
          None => {
            diag.add_error(DivideByZero, span);
            Failure(Integral::new(lhs.width(), 0, lhs.signedness()).into())
          },
        }
      }};
    }
    use Operator::*;
    match op {
      Plus => arith!(overflowing_add, Plus),
      Minus => arith!(overflowing_sub, Minus),
      Star => arith!(overflowing_mul, Star),

      Slash => div_zero!(checked_div),
      Percent => div_zero!(checked_rem),

      And => Success(Self::from_bool(!lhs.is_zero() && !rhs.is_zero())),
      Or => Success(Self::from_bool(!lhs.is_zero() || !rhs.is_zero())),

      Less => Success(Self::from_bool(lhs < rhs)),
      LessEqual => Success(Self::from_bool(lhs <= rhs)),
      Greater => Success(Self::from_bool(lhs > rhs)),
      GreaterEqual => Success(Self::from_bool(lhs >= rhs)),
      EqualEqual => Success(Self::from_bool(lhs == rhs)),
      NotEqual => Success(Self::from_bool(lhs != rhs)),

      Ampersand => Success(lhs & rhs),
      Pipe => Success(lhs | rhs),
      Caret => Success(lhs ^ rhs),
      _ => contract_violation!(
        "not a binary operator or bin-op but cannot be applied to integral! assignment op should be handled upstream, so does comma."
      ),
    }
  }
}

impl Floating {
  pub fn handle_binary_op(
    op: Operator,
    lhs: Floating,
    rhs: Floating,
    span: SourceSpan,
    diag: &impl Diagnosis,
  ) -> FoldingResult<Self> {
    debug_assert!(
      lhs.format() == rhs.format(),
      "Mismatched floating formats in binary operation"
    );
    debug_assert!(
      op.binary(),
      "Tried to perform unary operation in binary op handler"
    );
    todo!()
  }
}
