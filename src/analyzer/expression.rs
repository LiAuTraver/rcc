use ::strum_macros::Display;

use crate::{
  common::{
    environment::SymbolRef,
    error::Error,
    types::{Primitive, Promotion, QualifiedType, Qualifiers, Type, TypeInfo},
  },
  type_alias_expr,
};

type_alias_expr! {Expression, QualifiedType, Variable, ImplicitCast}
#[derive(Debug, Clone, Copy, Display, PartialEq)]
pub enum ValueCategory {
  #[strum(serialize = "lvalue")]
  LValue,
  /// 6.3.2: "rvalue" is in this document described as the "value of an expression".
  ///        which, is different from the one defined in C++ standard.
  #[strum(serialize = "rvalue")]
  RValue,
}
#[derive(Debug)]
pub struct Expression {
  raw_expr: RawExpr,
  expr_type: QualifiedType,
  value_category: ValueCategory,
}
impl Expression {
  pub fn new(raw_expr: RawExpr, expr_type: QualifiedType, value_category: ValueCategory) -> Self {
    Self {
      raw_expr,
      expr_type,
      value_category,
    }
  }
  pub fn unqualified_type(&self) -> &Type {
    &self.expr_type.unqualified_type
  }
  pub fn qualifiers(&self) -> &Qualifiers {
    &self.expr_type.qualifiers
  }
  pub fn qualified_type(&self) -> &QualifiedType {
    &self.expr_type
  }
  pub fn raw_expr(&self) -> &RawExpr {
    &self.raw_expr
  }
  pub fn value_category(&self) -> ValueCategory {
    self.value_category
  }
}
impl Primitive {
  #[must_use]
  pub fn common_type(lhs: Primitive, rhs: Primitive) -> Primitive {
    // If both operands have the same type, then no further conversion is needed.
    // first: _Decimal types ignored
    // also, complex types ignored
    if lhs == rhs {
      return lhs;
    }
    if matches!(lhs, Primitive::Void | Primitive::Nullptr)
      || matches!(rhs, Primitive::Void | Primitive::Nullptr)
    {
      panic!("Invalid types for common type: {:?}, {:?}", lhs, rhs);
    }
    // otherwise, if either operand is of some floating type, the other operand is converted to it.
    // Otherwise, if any of the two types is an enumeration, it is converted to its underlying type. - handled upstream
    match (lhs.is_floating_point(), rhs.is_floating_point()) {
      (true, false) => lhs,
      (false, true) => rhs,
      (true, true) => Primitive::common_floating_rank(lhs, rhs),
      (false, false) => Primitive::common_integer_rank(lhs, rhs),
    }
  }
  fn common_floating_rank(lhs: Primitive, rhs: Primitive) -> Primitive {
    assert!(lhs.is_floating_point() && rhs.is_floating_point());
    if lhs.floating_rank() > rhs.floating_rank() {
      lhs
    } else {
      rhs
    }
  }
  fn common_integer_rank(lhs: Primitive, rhs: Primitive) -> Primitive {
    assert!(lhs.is_integer() && rhs.is_integer());

    let (lhs, _) = lhs.integer_promotion();
    let (rhs, _) = rhs.integer_promotion();
    if lhs == rhs {
      // done
      return lhs;
    }
    if lhs.is_unsigned() == rhs.is_unsigned() {
      return if lhs.integer_rank() > rhs.integer_rank() {
        lhs
      } else {
        rhs
      };
    }
    let (unsigned_oprand, signed_oprand) = if lhs.is_unsigned() {
      (lhs, rhs)
    } else {
      (rhs, lhs)
    };

    if unsigned_oprand.integer_rank() >= signed_oprand.integer_rank() {
      unsigned_oprand
    } else if signed_oprand.size() > unsigned_oprand.size() {
      signed_oprand
    } else {
      // if the signed type cannot represent all values of the unsigned type, return the unsigned version of the signed type
      // the signed type is always larger than the corresponding unsigned type on my x86_64 architecture
      // so this branch is unlikely to be taken
      signed_oprand.into_unsigned()
    }
  }
}
impl Expression {
  pub fn is_lvalue(&self) -> bool {
    matches!(self.value_category, ValueCategory::LValue)
  }
  /// 6.3.2.1:  A modifiable lvalue is an lvalue that does not have array type, does not have an incomplete
  ///           type, does not have a const-qualified type, and if it is a structure or union, does not have any
  ///           member (including, recursively, any member or element of all contained aggregates or unions) with
  ///           a const-qualified type.
  pub fn is_modifiable_lvalue(&self) -> bool {
    self.is_lvalue() && self.qualified_type().is_modifiable()
  }
  pub fn to_rvalue(self) -> Self {
    Self {
      value_category: ValueCategory::RValue,
      ..self
    }
  }
  pub fn default_int() -> Self {
    Self {
      raw_expr: RawExpr::Constant(Constant::Int(0)),
      expr_type: QualifiedType::new(Qualifiers::empty(), Type::Primitive(Primitive::Int)),
      value_category: ValueCategory::RValue,
    }
  }
}

impl Expression {
  /// 6.3.1.8 Usual arithmetic conversions, applied implicitly where arithmetic conversions are required:
  /// `+`, `-`, `*`, `/`, `%`, `&`, `|`, `^`, `<<`, `>>`
  ///
  /// unary/binary.
  #[must_use]
  pub fn usual_arithmetic_conversion(self) -> Result<Self, Error> {
    let binary = match self.raw_expr {
      RawExpr::Binary(b) if !b.operator.is_right_associative() => b,
      RawExpr::Binary(b) => unreachable!("assignment operator should not reach here: {:?}", b),
      RawExpr::Unary(u) => {
        return Ok(u.expression.promote());
      }
      _ => {
        todo!()
      }
    };
    let Binary {
      left,
      operator,
      right,
    } = binary;
    let lhs = left.promote();
    let rhs = right.promote();
    // only conside primitive types here
    let common_type = match (
      &lhs.expr_type.unqualified_type,
      &rhs.expr_type.unqualified_type,
    ) {
      (Type::Primitive(l), Type::Primitive(r)) => Primitive::common_type(l.clone(), r.clone()),
      _ => {
        panic!(
          "usual arithmetic conversion only supports primitive types: {:?}, {:?}",
          lhs.expr_type.unqualified_type, rhs.expr_type.unqualified_type
        );
      }
    };
    let left = Self::cast_if_needed_unqual(lhs, Type::Primitive(common_type.clone()));
    let right = Self::cast_if_needed_unqual(rhs, Type::Primitive(common_type.clone()));
    Ok(Self::new(
      RawExpr::Binary(Binary::new(operator, left, right)),
      QualifiedType::new_unqualified(Type::Primitive(common_type)),
      ValueCategory::RValue,
    ))
  }
  fn cast_if_needed_unqual(expr: Expression, target_type: Type) -> Expression {
    if expr.unqualified_type() == &target_type {
      expr // no cast needed
    } else {
      Expression::new(
        RawExpr::ImplicitCast(ImplicitCast::new(expr)),
        QualifiedType::new_unqualified(target_type),
        ValueCategory::RValue,
      )
    }
  }
  pub fn promote(self) -> Self {
    let (promoted_type, changed) = self.expr_type.clone().promote();
    if changed {
      Self::new(
        RawExpr::ImplicitCast(ImplicitCast::new(self)),
        promoted_type,
        ValueCategory::RValue,
      )
    } else {
      self
    }
  }
  /// 6.3.1.2.1: When any scalar value is converted to bool, the result is false if:
  ///   - the value is a zero (for arithmetic types)
  ///   - null (for pointer types),
  ///   - the scalar has type nullptr_t
  ///
  /// otherwise, the result is true.
  ///
  /// NO promotion is performed.
  ///
  /// unary.
  #[must_use]
  pub fn conditional_conversion(self) -> Self {
    let decayed = self.decay();
    match decayed.expr_type.unqualified_type {
      Type::Primitive(Primitive::Bool) => decayed,
      Type::Primitive(_) => Self::new(
        RawExpr::ImplicitCast(ImplicitCast::new(decayed)),
        QualifiedType::new_unqualified(Type::Primitive(Primitive::Bool)),
        ValueCategory::RValue,
      ),
      // compare with nullptr
      Type::Pointer(_) => Self::new(
        RawExpr::ImplicitCast(ImplicitCast::new(decayed)),
        QualifiedType::new_unqualified(Type::Primitive(Primitive::Bool)),
        ValueCategory::RValue,
      ),

      Type::Array(array) => {
        panic!(
          "should be decayed before conditional conversion: {:?}",
          array
        )
      }
      Type::FunctionProto(function_proto) => panic!(
        "should be decayed before conditional conversion: {:?}",
        function_proto
      ),
      Type::Enum(_) | Type::Record(_) | Type::Union(_) => {
        todo!("conditional conversion for complex types")
      }
    }
  }
  /// If an expression of any other type is evaluated as a void expression, its value or designator is discarded.
  /// (A void expression is evaluated for its side effects.)
  ///
  /// unary.
  #[must_use]
  pub fn void_conversion(self) -> Self {
    let target_type = QualifiedType::new_unqualified(Type::Primitive(Primitive::Void));
    Self::new(
      RawExpr::ImplicitCast(ImplicitCast::new(self)),
      target_type,
      ValueCategory::RValue,
    )
  }
  #[must_use]
  pub fn decay(self) -> Self {
    todo!()
  }
}
impl ::core::default::Default for Expression {
  fn default() -> Self {
    Self {
      raw_expr: RawExpr::Empty,
      expr_type: QualifiedType::new(Qualifiers::empty(), Type::Primitive(Primitive::Void)),
      value_category: ValueCategory::RValue,
    }
  }
}
#[derive(Debug)]
pub struct Variable {
  pub name: SymbolRef,
}
impl Variable {
  pub fn new(name: SymbolRef) -> Self {
    Self { name }
  }
}
#[derive(Debug)]
pub struct ImplicitCast {
  pub expr: Box<Expression>,
}
impl ImplicitCast {
  pub fn new(expr: Expression) -> Self {
    Self {
      expr: Box::new(expr),
    }
  }
}
mod fmt {

  use super::{ImplicitCast, Variable};
  use ::std::fmt::Display;

  use super::Expression;

  impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{}", self.raw_expr)
    }
  }
  // the "specialization" for the smart pointer case
  impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{}", self.name.borrow())
    }
  }
  impl Display for ImplicitCast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{}", self.expr)
    }
  }
}

mod test {

  #[test]
  fn add_int_float() {
    use super::*;
    use crate::common::operator::Operator;

    let int_expr = Expression::new(
      RawExpr::Constant(Constant::Int(42)),
      QualifiedType::new_unqualified(Type::from(Primitive::Int)),
      ValueCategory::RValue,
    );
    let float_expr = Expression::new(
      RawExpr::Constant(Constant::Float(3.14)),
      QualifiedType::new_unqualified(Type::from(Primitive::Float)),
      ValueCategory::RValue,
    );
    let expr = Expression::new(
      RawExpr::Binary(Binary::new(Operator::Plus, int_expr, float_expr)),
      QualifiedType::new_unqualified(Type::from(Primitive::Float)),
      ValueCategory::RValue,
    );
    let promoted_expr = expr.usual_arithmetic_conversion();
    // type shall be
    println!("Promoted expression: {:#?}", promoted_expr);
  }
}
