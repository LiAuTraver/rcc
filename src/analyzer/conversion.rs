use crate::analyzer::expression::{Binary, Expression, ImplicitCast, RawExpr, ValueCategory};
use crate::common::error::Error;
use crate::common::types::{
  CastType, Pointer, Primitive, Promotion, QualifiedType, Qualifiers, Type,
};

impl Expression {
  /// 6.3.1.8 Usual arithmetic conversions, applied implicitly where arithmetic conversions are required:
  /// `+`, `-`, `*`, `/`, `%`, `&`, `|`, `^`, `<<`, `>>`
  ///
  /// unary/binary.
  #[must_use]
  pub fn usual_arithmetic_conversion(
    lhs: Expression,
    rhs: Expression,
  ) -> Result<(Expression, Expression, QualifiedType), Error> {
    let lhs = lhs.promote();
    let rhs = rhs.promote();

    let common_type = match (lhs.unqualified_type(), rhs.unqualified_type()) {
      (Type::Primitive(l), Type::Primitive(r)) => Primitive::common_type(l.clone(), r.clone()),
      _ => return Err(()),
    };

    let common_qtype = QualifiedType::new_unqualified(Type::Primitive(common_type));
    let lhs = Self::cast_if_needed_unqual(lhs, common_qtype.unqualified_type.clone());
    let rhs = Self::cast_if_needed_unqual(rhs, common_qtype.unqualified_type.clone());

    Ok((lhs, rhs, common_qtype))
  }
  fn cast_if_needed_unqual(expr: Expression, target_type: Type) -> Expression {
    if expr.unqualified_type() == &target_type {
      expr // no cast needed
    } else {
      let cast_type = match (&expr.unqualified_type(), &target_type) {
        (Type::Primitive(from), Type::Primitive(to)) => {
          if from.is_integer() && to.is_integer() {
            CastType::IntegralCast
          } else if from.is_integer() && to.is_floating_point() {
            CastType::IntegralToFloating
          } else if from.is_floating_point() && to.is_integer() {
            CastType::FloatingToIntegral
          } else if from.is_floating_point() && to.is_floating_point() {
            CastType::FloatingCast
          } else {
            panic!("Invalid cast: {:?} -> {:?}", from, to)
          }
        }
        _ => panic!(
          "Invalid cast: {:?} -> {:?}",
          expr.unqualified_type(),
          target_type
        ),
      };

      Expression::new_rvalue(
        RawExpr::ImplicitCast(ImplicitCast::new(expr.into(), cast_type)),
        QualifiedType::new_unqualified(target_type),
      )
    }
  }
  pub fn promote(self) -> Self {
    let (promoted_type, cast_type) = self.expr_type.clone().promote();
    match cast_type {
      CastType::Noop => self,
      cast_type => Self::new_rvalue(
        RawExpr::ImplicitCast(ImplicitCast::new(self.into(), cast_type)),
        promoted_type,
      ),
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
  pub fn conditional_conversion(self) -> Result<Self, Error> {
    let decayed = self.decay();
    match decayed.expr_type.unqualified_type {
      Type::Primitive(Primitive::Bool) => Ok(decayed),
      Type::Primitive(ref p) if p.is_integer() => Ok(Self::new_rvalue(
        RawExpr::ImplicitCast(ImplicitCast::new(
          decayed.into(),
          CastType::IntegralToBoolean,
        )),
        QualifiedType::new_unqualified(Type::Primitive(Primitive::Bool)),
      )),
      Type::Primitive(ref p) if p.is_floating_point() => Ok(Self::new_rvalue(
        RawExpr::ImplicitCast(ImplicitCast::new(
          decayed.into(),
          CastType::FloatingToBoolean,
        )),
        QualifiedType::new_unqualified(Type::Primitive(Primitive::Bool)),
      )),
      Type::Primitive(Primitive::Nullptr) => Ok(Self::new_rvalue(
        RawExpr::ImplicitCast(ImplicitCast::new(
          decayed.into(),
          CastType::NullptrToBoolean,
        )),
        QualifiedType::new_unqualified(Type::Primitive(Primitive::Bool)),
      )),
      Type::Primitive(Primitive::Void) => {
        Err(()) // void cannot be converted to bool
      }
      Type::Primitive(_) => {
        Err(()) // other primitive types cannot be converted to bool
      }
      // compare with nullptr
      Type::Pointer(_) => Ok(Self::new_rvalue(
        RawExpr::ImplicitCast(ImplicitCast::new(
          decayed.into(),
          CastType::PointerToBoolean,
        )),
        QualifiedType::new_unqualified(Type::Primitive(Primitive::Bool)),
      )),

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
    Self::new_rvalue(
      RawExpr::ImplicitCast(ImplicitCast::new(self.into(), CastType::ToVoid)),
      target_type,
    )
  }
  #[must_use]
  pub fn decay(self) -> Self {
    match self.unqualified_type() {
      Type::Array(_) => self.array_to_pointer_decay(),
      Type::FunctionProto(_) => self.function_to_pointer_decay(),
      _ => self,
    }
  }
  /// A function designator is an expression that has function type. Except when it is the operand of the
  ///     sizeof operator,56) a typeof operator, or the unary & operator, a function designator with type
  ///     "function returning type" is converted to an expression that has type "pointer to function returning
  ///     type"
  #[must_use]
  pub fn function_to_pointer_decay(self) -> Self {
    let function_type = match self.unqualified_type() {
      Type::FunctionProto(f) => f,
      _ => unreachable!(),
    };
    assert!(
      self.qualifiers().is_empty(),
      "function type should not have qualifiers: {:?}",
      self.expr_type
    );
    let pointer_type = Type::from(Pointer::new(
      QualifiedType::new(
        self.qualifiers().clone(), // should equal to empty -- functionproto never has qualifiers
        Type::FunctionProto(function_type.clone()),
      )
      .into(),
    ));
    Self::new_rvalue(
      RawExpr::ImplicitCast(ImplicitCast::new(
        self.into(),
        CastType::FunctionToPointerDecay,
      )),
      // The pointer itself is never qualified
      QualifiedType::new_unqualified(pointer_type),
    )
  }
  /// Except when it is the operand of the sizeof operator, or typeof operators, or the unary & operator,
  ///       or is a string literal used to initialize an array, an expression that has type "array of type" is converted
  ///       to an expression with type "pointer to type" that points to the initial element of the array object and
  ///       is not an lvalue.
  #[must_use]
  pub fn array_to_pointer_decay(self) -> Self {
    let array_type = match self.unqualified_type() {
      Type::Array(a) => a,
      _ => unreachable!(),
    };
    assert!(
      self.qualifiers().is_empty(),
      "array type should not have qualifiers: {:?}",
      self.expr_type
    );
    let pointer_type = Type::from(Pointer::new(
      // array itself should not have qualifiers, but the element qualifiers are preserved
      array_type.element_type.clone().into(),
    ));
    Self::new_rvalue(
      RawExpr::ImplicitCast(ImplicitCast::new(
        self.into(),
        CastType::ArrayToPointerDecay,
      )),
      // The pointer itself is never qualified
      QualifiedType::new_unqualified(pointer_type),
    )
  }
  #[must_use]
  pub fn assignment_conversion(self, target_type: &QualifiedType) -> Result<Self, Error> {
    todo!()
  }
  /// 6.3.2.1 Lvalues, arrays, and function designators
  ///
  /// Except when it is the operand of the sizeof operator, or the typeof operators, the unary `&` operator,
  ///     the `++` operator, the `--` operator, or the left operand of the `.` operator or an assignment operator, an
  ///     lvalue that does not have array type is converted to the value stored in the designated object (and is
  ///     no longer an lvalue); this is called lvalue conversion.
  #[must_use]
  pub fn lvalue_conversion(self) -> Self {
    if self.is_lvalue() {
      // If the lvalue has qualified type, the value has the unqualified version of the type of the lvalue. perform cast from lvalue to rvalue
      let old_unqual_type = self.expr_type.unqualified_type.clone();
      Self::new_rvalue(
        RawExpr::ImplicitCast(ImplicitCast::new(self.into(), CastType::LValueToRValue)),
        QualifiedType::new_unqualified(old_unqual_type),
      )
    } else {
      self
    }
  }
}
