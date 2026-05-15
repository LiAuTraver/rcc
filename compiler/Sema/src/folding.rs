//! FIXME: many redundant helpers like [`Expression::fold`], [`Folder::doit`], [`Folder::child`]...

use ::rcc_adt::{Floating, Integral, Size};
use ::rcc_ast::types::{CastType, Compatibility, QualifiedType, TypeInfo};
use ::rcc_shared::{
  DiagData::{self, *},
  Operator, OperatorCategory, SourceSpan,
};
use ::rcc_utils::{RefEq, contract_assert, contract_violation};
use ::std::ops::Deref;

use super::{
  Sema,
  declaration::Initializer,
  expression::{
    ArraySubscript, Binary, CStyleCast, Call, CompoundAssign, CompoundLiteral,
    Constant, Empty, ExprRef, Expression, ImplicitCast, MemberAccess, Paren,
    RawExpr, SizeOf, SizeOfKind, Ternary, Unary, ValueCategory, Variable,
  },
};

impl<'c> Expression<'c> {
  #[inline(always)]
  fn fold(&'c self, folder: &Folder<'_, 'c>) -> FoldingResult<'c> {
    Folder::child(folder, self).doit()
  }
}

type FR<'c> = Option<ExprRef<'c>>;
pub type FoldingResult<'c> = FR<'c>;
pub struct Folder<'i, 'c>
where
  'c: 'i,
{
  sema: &'i Sema<'c>,
  expression: ExprRef<'c>,

  /// whether to fold global static const non-volatile variable as C23/C++ `constexpr`-like variable, like:
  /// ```c
  /// static const auto len = 10;
  /// int arr[len];
  /// ```
  /// VLA arr would be fold to fix-sized array:
  /// ```c
  /// int arr[10];
  /// ```
  /// This is for the good of passing SysY OJ(how pethatic to say it is a subset of C...),
  /// as a workaround we always set it to true and prob emit a warning.
  ///
  ///
  /// Caveat: should still accept address constant
  /// ```c
  /// const auto p = &len; // Ok.
  /// ```
  /// and probably keep to reject
  /// ```c
  /// static constexpr auto len2 = 10;
  /// static const auto p = &len2; // Error.
  /// ```
  /// This differs in C and C++, not sure fr.
  relaxed_static_const_var: bool,
}
impl<'i, 'c> Deref for Folder<'i, 'c> {
  type Target = Sema<'c>;

  #[inline(always)]
  fn deref(&self) -> &'i Self::Target {
    self.sema
  }
}

impl<'i, 'c> Folder<'i, 'c> {
  #[inline(always)]
  pub fn new(
    sema: &'i Sema<'c>,
    relaxed_static_const_var: bool,
    expression: ExprRef<'c>,
  ) -> Self {
    Self {
      sema,
      expression,
      relaxed_static_const_var,
    }
  }

  #[inline(always)]
  fn child<'a>(parent: &'a Self, expression: ExprRef<'c>) -> Self
  where
    'a: 'i,
  {
    Self::new(parent, parent.relaxed_static_const_var, expression)
  }
}
impl<'c> Folder<'_, 'c> {
  #[inline(always)]
  pub fn doit(self) -> FR<'c> {
    // dont judge by is_modifiable_lvalue here; it is always likely to be one before folding.
    self.fold(&**self.expression)
  }

  #[inline(always)]
  fn span(&self) -> SourceSpan {
    self.expression.span()
  }

  #[inline(always)]
  fn qualified_type(&self) -> &QualifiedType<'c> {
    self.expression.qualified_type()
  }

  #[inline(always)]
  fn value_category(&self) -> ValueCategory {
    self.expression.value_category()
  }

  #[inline(always)]
  pub fn add_error(&self, error: DiagData<'c>) {
    self.sema.add_error(error, self.expression.span());
  }

  #[inline(always)]
  pub fn add_warning(&self, warning: DiagData<'c>) {
    self.sema.add_warning(warning, self.expression.span());
  }

  #[inline(always)]
  pub fn new_expr(&self, variant: impl Into<RawExpr<'c>>) -> ExprRef<'c> {
    Expression::new(
      self,
      variant,
      *self.qualified_type(),
      self.value_category(),
      self.span(),
    )
  }

  #[inline(always)]
  pub fn new_constant(&self, variant: impl Into<Constant<'c>>) -> ExprRef<'c> {
    self.new_expr(variant.into())
  }
}
impl<'c> Folder<'_, 'c> {
  fn integral_binary_op(
    &self,
    op: Operator,
    lhs: Integral,
    rhs: Integral,
  ) -> Integral {
    debug_assert!(op.binary());
    debug_assert_eq!(lhs.width(), rhs.width());
    debug_assert_eq!(lhs.signedness(), rhs.signedness());

    macro_rules! arith {
      ($func:ident, $op:expr) => {{
        let (result, overflow) = lhs.$func(rhs);
        if overflow {
          self.add_warning(ArithmeticBinOpOverflow(
            (lhs.into(), rhs.into(), $op).into(),
          ));
        }
        result
      }};
    }
    macro_rules! div_zero {
      ($func:ident) => {{
        lhs.$func(rhs).unwrap_or_else(|| {
          self.add_error(DivideByZero);
          Integral::new(0, lhs.width(), lhs.signedness()).into()
        })
      }};
    }
    macro_rules! logical_misuse_warn {
    ($op_sym:tt, $op_variant:ident, $suggest:ident) => {{
        self.add_warning(
            LogicalOpMisuse($op_variant, $suggest.into()),
        );
        Integral::from_bool(!lhs.is_zero() $op_sym !rhs.is_zero())
    }}
    }
    use Operator::*;
    match op {
      Plus => arith!(overflowing_add, Plus),
      Minus => arith!(overflowing_sub, Minus),
      Star => arith!(overflowing_mul, Star),

      Slash => div_zero!(checked_div),
      Percent => div_zero!(checked_rem),

      LogicalAnd => logical_misuse_warn!(&&, LogicalAnd, Ampersand),
      LogicalOr => logical_misuse_warn!(||, LogicalOr, Pipe),

      Less => Integral::from_bool(lhs < rhs),
      LessEqual => Integral::from_bool(lhs <= rhs),
      Greater => Integral::from_bool(lhs > rhs),
      GreaterEqual => Integral::from_bool(lhs >= rhs),
      EqualEqual => Integral::from_bool(lhs == rhs),
      NotEqual => Integral::from_bool(lhs != rhs),

      Ampersand => lhs & rhs,
      Pipe => lhs | rhs,
      Caret => lhs ^ rhs,
      _ => contract_violation!(
        "not a binary operator or bin-op but cannot be applied to integral! \
         assignment op should be handled upstream, so does comma."
      ),
    }
  }

  fn integral_unary_op(
    &self,
    operator: Operator,
    operand: Integral,
  ) -> Integral {
    debug_assert!(operator.unary());
    use Operator::*;
    match operator {
      Plus => operand,
      Minus => -operand,
      Not => Integral::from_bool(operand.is_zero()),
      Tilde => !operand, // `!` is bitwise NOT here; rust does not have `~` operator
      Star | Ampersand | PlusPlus | MinusMinus =>
        contract_violation!("unary operator not applicable to integral!"),
      _ => unreachable!(),
    }
  }

  fn floating_binary_arith_op(
    &self,
    op: Operator,
    lhs: Floating,
    rhs: Floating,
  ) -> Floating {
    use Operator::*;
    debug_assert!(
      op.binary() && op != Percent,
      "Tried to perform unary operation in binary op handler"
    );
    macro_rules! arith {
      ($op:tt) => {{
        let res = lhs $op rhs;
        if lhs.is_finite() && rhs.is_finite() && res.is_infinite() {
          self.add_warning(
            ArithmeticBinOpOverflow((lhs.into(), rhs.into(), op).into()),
          );
        }
        res
      }};
    }

    match op {
      Plus => arith!(+),
      Minus => arith!(-),
      Star => arith!(*),
      Slash => arith!(/),
      _ => contract_violation!(
        "not a binary arithmetic operator but cannot be applied to floating! \
         assignment op should be handled upstream, so does comma."
      ),
    }
  }

  fn floating_binary_rel_op(
    &self,
    op: Operator,
    lhs: Floating,
    rhs: Floating,
  ) -> Integral {
    use OperatorCategory::*;
    debug_assert!(
      matches!(op.category(), Logical | Relational),
      "Tried to perform unary operation in binary op handler"
    );
    macro_rules! logical_misuse_warn {
    ($op_sym:tt, $op_variant:ident, $suggest:ident) => {{
        self.add_warning(
            LogicalOpMisuse($op_variant, $suggest.into()),
        );
        Integral::from_bool(!lhs.is_zero() $op_sym !rhs.is_zero())
    }}
    }
    use Operator::*;
    match op {
      LogicalAnd => logical_misuse_warn!(&&, LogicalAnd, Ampersand),
      LogicalOr => logical_misuse_warn!(||, LogicalOr, Pipe),

      Less => Integral::from_bool(lhs < rhs),
      LessEqual => Integral::from_bool(lhs <= rhs),
      Greater => Integral::from_bool(lhs > rhs),
      GreaterEqual => Integral::from_bool(lhs >= rhs),
      EqualEqual => Integral::from_bool(lhs == rhs),
      NotEqual => Integral::from_bool(lhs != rhs),
      _ => contract_violation!(
        "not a binary operator or bin-op but cannot be applied to floating! \
         {op}"
      ),
    }
  }

  fn floating_unary_arith_op(
    &self,
    operator: Operator,
    operand: Floating,
  ) -> Floating {
    debug_assert!(operator.unary());
    use Operator::*;
    match operator {
      Plus => operand,
      Minus => -operand,
      Not | Tilde | Star | Ampersand | PlusPlus | MinusMinus =>
        contract_violation!(
          "unary operator not applicable to floating! should be handled \
           upstream"
        ),
      _ => unreachable!(),
    }
  }

  fn floating_unary_order_op(
    &self,
    operator: Operator,
    operand: Floating,
  ) -> Integral {
    debug_assert!(operator.unary());
    use Operator::*;
    match operator {
      Not => {
        self.add_warning(LogicalOpMisuse(Not, None));
        Integral::from_bool(operand.is_zero())
      },
      LogicalAnd | LogicalOr | Less | LessEqual | Greater | GreaterEqual
      | EqualEqual | NotEqual | Star | Ampersand | PlusPlus | MinusMinus =>
        contract_violation!(
          "unary operator not applicable to floating! should be handled \
           upstream"
        ),
      _ => unreachable!(),
    }
  }
}
pub trait Fold<'c, VariantTy> {
  #[must_use]
  fn fold(&self, variant: &VariantTy) -> FR<'c>;
}
impl<'c> Fold<'c, RawExpr<'c>> for Folder<'_, 'c> {
  fn fold(&self, raw_expr: &RawExpr<'c>) -> FR<'c> {
    ::rcc_utils::static_dispatch!(
      RawExpr: raw_expr,
      |variant| <Self as Fold<'c, _>>::fold(self, variant) =>
      Empty Constant Unary Binary Call Paren MemberAccess Ternary SizeOf
      CStyleCast ArraySubscript CompoundLiteral Variable ImplicitCast CompoundAssign
    )
  }
}
impl<'c> Fold<'c, Empty> for Folder<'_, 'c> {
  #[inline(always)]
  fn fold(&self, _empty: &Empty) -> FR<'c> {
    None
  }
}
impl<'c> Fold<'c, Constant<'c>> for Folder<'_, 'c> {
  fn fold(&self, _constant: &Constant<'c>) -> FR<'c> {
    Some(self.expression)
  }
}
impl<'c> Fold<'c, Unary<'c>> for Folder<'_, 'c> {
  fn fold(&self, unary: &Unary<'c>) -> FR<'c> {
    debug_assert!(
      unary.operator.unary(),
      "not an unary operator! should not happen!"
    );
    let folded_operand = unary.operand.fold(self)?;

    contract_assert!(
      folded_operand.is_constant(),
      "only implemented for constant var of constant eval"
    );
    use Constant::*;
    use OperatorCategory::*;

    let constant: Constant =
      match folded_operand.as_constant_unchecked().clone() {
        Integral(operand) =>
          self.integral_unary_op(unary.operator, operand).into(),
        Floating(operand) => match unary.operator.category() {
          Arithmetic =>
            self.floating_unary_arith_op(unary.operator, operand).into(),
          Logical =>
            self.floating_unary_order_op(unary.operator, operand).into(),
          _ => contract_violation!(
            "not a unary operator or un-op but cannot be applied to floating!"
          ),
        },
        _ => unimplemented!(),
      };
    Some(self.new_constant(constant))
  }
}
impl<'c> Fold<'c, Binary<'c>> for Folder<'_, 'c> {
  fn fold(&self, binary: &Binary<'c>) -> FR<'c> {
    debug_assert!(
      binary.operator.binary(),
      "not a binary operator! should not happen!"
    );
    // logical && and || are short-circuit evaluated,
    // so    `static const int j = 0 && x;` would pass constant folding,
    // while `static const int j = 0 || x;` would not.
    let fl = binary.left.fold(self);
    let fr = binary.right.fold(self);

    use Operator::{Comma, LogicalAnd, LogicalOr};

    let (folded_lhs, folded_rhs) = match (fl, fr) {
      (Some(_), Some(right)) if binary.operator == Comma => {
        self.add_warning(LeftCommaNoEffect);
        // safely return right, here left has already been evaluated...
        return Some(right);
      },
      (Some(left), Some(right)) => (left, right),
      (Some(left), None)
        if binary.operator == LogicalAnd
          && left.as_constant_unchecked().is_zero() =>
        return Some(left),

      (Some(_), None) if binary.operator == LogicalAnd => return None,

      (Some(left), None)
        if binary.operator == LogicalOr
          && left.as_constant_unchecked().is_not_zero() =>
        return Some(left),
      (Some(_), None) if binary.operator == LogicalOr => return None,

      _ => return None,
    };

    assert!(
      folded_lhs.is_constant() && folded_rhs.is_constant(),
      "only implemented for constant var of constant eval"
    );
    assert!(
      RefEq::ref_eq(
        folded_lhs.unqualified_type(),
        folded_rhs.unqualified_type()
      ),
      "type checker makes sure both sides have the same type via \
       `ImplicitCast`! {:#?} vs {:#?}, op {:#?}",
      folded_lhs.unqualified_type(),
      folded_rhs.unqualified_type(),
      binary.operator
    );
    let lhs_type = folded_lhs.qualified_type();
    let rhs_type = folded_rhs.qualified_type();
    let lhs_value_category = folded_lhs.value_category();
    let rhs_value_category = folded_rhs.value_category();

    assert!(
      RefEq::ref_eq(lhs_type.unqualified_type, rhs_type.unqualified_type),
      "type checker ensures both sides have the same types!"
    );

    assert!(
      lhs_value_category == ValueCategory::RValue
        && rhs_value_category == ValueCategory::RValue,
      "type checker ensures both sides are rvalues! 
      Assignment is handled at start of this function."
    );

    let lhs = folded_lhs.as_constant_unchecked().clone();
    let rhs = folded_rhs.as_constant_unchecked().clone();

    use Constant::*;
    use OperatorCategory::*;
    let constant: Constant = match (lhs, rhs) {
      (Integral(lhs), Integral(rhs)) =>
        self.integral_binary_op(binary.operator, lhs, rhs).into(),
      (Floating(lhs), Floating(rhs)) => match binary.operator.category() {
        Logical | Relational => self
          .floating_binary_rel_op(binary.operator, lhs, rhs)
          .into(),
        Arithmetic => self
          .floating_binary_arith_op(binary.operator, lhs, rhs)
          .into(),
        _ => contract_violation!(
          "not a binary operator or bin-op but cannot be applied to floating!"
        ),
      },
      (String(_), String(_)) => contract_violation!("can we reach here?"),
      (Nullptr(), Nullptr()) => contract_violation!("can we reach here?"),
      _ => contract_violation!(
        "type checker ensures both sides have the same types! or \
         unimplemented type"
      ),
    };

    Some(self.new_constant(constant))
  }
}
impl<'c> Fold<'c, Call<'c>> for Folder<'_, 'c> {
  #[inline(always)]
  fn fold(&self, _call: &Call<'c>) -> FR<'c> {
    None
  }
}
impl<'c> Fold<'c, Paren<'c>> for Folder<'_, 'c> {
  #[inline(always)]
  fn fold(&self, paren: &Paren<'c>) -> FR<'c> {
    paren.expr.fold(self)
  }
}
impl<'c> Fold<'c, MemberAccess<'c>> for Folder<'_, 'c> {
  fn fold(&self, _member_access: &MemberAccess<'c>) -> FR<'c> {
    unimplemented!()
  }
}
impl<'c> Fold<'c, Ternary<'c>> for Folder<'_, 'c> {
  fn fold(&self, ternary: &Ternary<'c>) -> FR<'c> {
    debug_assert!(
      ternary
        .then_expr
        .expect("?: unimplemented")
        .qualified_type()
        .compatible_with(ternary.else_expr.qualified_type()),
      "type checker ensures both branches have compatible types!"
    );

    let fc = ternary.condition.fold(self)?;
    match fc.as_constant_unchecked().is_zero() {
      true => ternary.else_expr.fold(self),
      false => ternary.then_expr.expect("?: unimplemented").fold(self),
    }
  }
}
impl<'c> Fold<'c, SizeOf<'c>> for Folder<'_, 'c> {
  fn fold(&self, sizeof: &SizeOf<'c>) -> FR<'c> {
    match sizeof.sizeof {
      SizeOfKind::Expression(expression) => expression.fold(self),
      SizeOfKind::Type(qualified_type) =>
        Some(self.new_constant(Integral::from_unsigned(
          if !qualified_type.is_complete(self) {
            self.add_error(IncompleteType(qualified_type.to_string()));

            Size::U0.to_builtin::<usize>()
          } else {
            qualified_type.size(self).to_builtin::<usize>()
          },
          self.pointer.size_bits(),
        ))),
    }
  }
}
impl<'c> Fold<'c, CStyleCast<'c>> for Folder<'_, 'c> {
  fn fold(&self, _cstyle_cast: &CStyleCast<'c>) -> FR<'c> {
    unimplemented!()
  }
}
impl<'c> Fold<'c, ArraySubscript<'c>> for Folder<'_, 'c> {
  /// always fails folding in C, unlike C++.
  #[inline(always)]
  fn fold(&self, _array_subscript: &ArraySubscript<'c>) -> FR<'c> {
    None
  }
}
impl<'c> Fold<'c, CompoundLiteral> for Folder<'_, 'c> {
  fn fold(&self, _compound_literal: &CompoundLiteral) -> FR<'c> {
    unimplemented!()
  }
}
impl<'c> Fold<'c, Variable<'c>> for Folder<'_, 'c> {
  fn fold(&self, variable: &Variable<'c>) -> FR<'c> {
    use ::rcc_ast::types::Qualifiers;
    let v = variable.as_variable()?; // unimplemented fror function
    match v.storage {
      _ if self.expression.is_modifiable_lvalue(self)
        || variable.qualified_type.contains(Qualifiers::Volatile) =>
        None,
      _ if !v.is_named_constant
        && !(variable.qualified_type.contains(Qualifiers::Const)
          && self.relaxed_static_const_var) =>
        None,
      _ => {
        let initializer = self
          .environment
          .find(variable.name)?
          .as_variable()
          .expect(
            "v is variable, semaa ensures the redeclarable chain of v is also \
             variable",
          )
          .initializer
          .as_ref()?; //< covers the case of var either is extern and referencing other TU or has no initializer)
        match initializer {
          Initializer::Scalar(expression) => expression
            .as_constant()
            .map(|constant| self.new_constant(constant.clone())),
          // unimplemented vvv
          Initializer::List(_) => None,
        }
      },
    }
  }
}
impl<'c> Fold<'c, ImplicitCast<'c>> for Folder<'_, 'c> {
  fn fold(&self, implicit_cast: &ImplicitCast<'c>) -> FR<'c> {
    let folded_expr = implicit_cast.expr.fold(self)?;

    let retype_or_reuse = || {
      if RefEq::ref_eq(self.qualified_type(), folded_expr.qualified_type())
        && self.value_category() == folded_expr.value_category()
      {
        // nothing changed, reuse the original expr node.
        folded_expr
      } else {
        self.new_expr((**folded_expr).clone())
      }
    };

    debug_assert!(
      !matches!(implicit_cast.cast_type, IntegralToPointer),
      "no such implicit cast! only for explicit."
    );

    use CastType::*;
    match implicit_cast.cast_type {
      Noop | ToVoid | LValueToRValue | BitCast => Some(retype_or_reuse()),
      PointerToIntegral | PointerToBoolean => None,
      NullptrToPointer => Some(retype_or_reuse()),
      IntegralCast => Some(
        self.new_constant(
          folded_expr
            .as_constant_unchecked()
            .as_integral_unchecked()
            .cast(
              self
                .qualified_type()
                .as_primitive_unchecked()
                .size_bits(self),
              folded_expr
                .qualified_type()
                .as_primitive_unchecked()
                .signedness(self)
                .expect("integral always has signedness"),
            ),
        ),
      ),
      // integral are promoted previously.
      IntegralToFloating => Some(
        self.new_constant(
          folded_expr
            .as_constant_unchecked()
            .as_integral_unchecked()
            .to_floating(
              self
                .qualified_type()
                .as_primitive_unchecked()
                .floating_format(),
            ),
        ),
      ),
      IntegralToBoolean => Some(
        self.new_constant(Integral::from_bool(
          !folded_expr
            .as_constant_unchecked()
            .as_integral_unchecked()
            .is_zero(),
        )),
      ),
      FloatingToBoolean => Some(
        self.new_constant(Integral::from_bool(
          !folded_expr
            .as_constant_unchecked()
            .as_floating_unchecked()
            .is_zero(),
        )),
      ),
      FloatingCast => Some(
        self.new_constant(
          folded_expr
            .as_constant_unchecked()
            .as_floating_unchecked()
            .cast(
              self
                .qualified_type()
                .as_primitive_unchecked()
                .floating_format(),
            ),
        ),
      ),
      FloatingToIntegral => Some(
        self.new_constant(
          folded_expr
            .as_constant_unchecked()
            .as_floating_unchecked()
            .to_integral(
              self
                .qualified_type()
                .as_primitive_unchecked()
                .size_bits(self),
              self
                .qualified_type()
                .as_primitive_unchecked()
                .signedness(self)
                .expect("floating point always has signedness"),
            ),
        ),
      ),
      IntegralToPointer => unimplemented!("address constant"),
      ArrayToPointerDecay => unimplemented!("address constant"),
      FunctionToPointerDecay => unimplemented!("address constant"),
    }
  }
}
impl<'c> Fold<'c, CompoundAssign<'c>> for Folder<'_, 'c> {
  /// should always fail in C.
  #[inline(always)]
  fn fold(&self, _compound_assign: &CompoundAssign<'c>) -> FR<'c> {
    None
  }
}
