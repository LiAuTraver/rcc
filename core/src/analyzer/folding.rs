use crate::{
  analyzer::expression::{Expression, RawExpr, SizeOfKind, Variable},
  common::{Operator, Storage},
  diagnosis::DiagData,
  types::Type,
};
/// FIXME: should implement this as `try_fold` or so, rather than `is_constant` methods
impl Expression {
  /// 6.6.8: An integer constant expression shall have integer type and shall only have operands that are
  ///           integer constants, named and compound literal constants of integer type, character constants,
  ///           sizeof expressions whose results are integer constants, alignof expressions, and floating, named,
  ///           or compound literal constants of arithmetic type that are the immediate operands of casts. Cast
  ///           operators in an integer constant expression shall only convert arithmetic types to integer types,
  ///           except as part of an operand to the typeof operators, sizeof operator, or alignof operator.
  pub fn is_integer_constant(&self) -> bool {
    match self.raw_expr() {
      RawExpr::Constant(c) => c.is_integer() || c.is_char_array(),
      // ignore VLA
      RawExpr::SizeOf(sizeof) =>
        if let SizeOfKind::Expression(e) = &sizeof.sizeof {
          e.unqualified_type().is_integer()
        } else {
          true // sizeof(type) is always constant
        },
      RawExpr::CStyleCast(cast) => cast.expr.is_integer_constant(),
      RawExpr::Unary(unary) =>
        matches!(unary.operator, Operator::Plus | Operator::Minus)
          && unary.operand.is_integer_constant(),
      RawExpr::Variable(variable) =>
        Self::is_named_integer_constant_unchecked(variable),
      _ => false,
    }
  }

  // todo: enum constant
  fn is_named_integer_constant(&self) -> bool {
    match self.raw_expr() {
      RawExpr::Variable(variable) =>
        Self::is_named_integer_constant_unchecked(variable),
      _ => false,
    }
  }

  fn is_named_integer_constant_unchecked(variable: &Variable) -> bool {
    let sym = variable.name.borrow();

    (sym.qualified_type.unqualified_type().is_integer()
      || sym.qualified_type.unqualified_type().as_array().is_some())
      && matches!(sym.storage_class, Storage::Constexpr)
  }

  /// 6.6.7
  pub fn is_named_constant(&self) -> bool {
    self.is_named_integer_constant() // this is incorrect, but ill leave it for now
  }

  /// 6.6.11: An address constant is a null pointer, a pointer to an lvalue designating an object of static storage
  ///   duration, or a pointer to a function designator; it shall be created explicitly using the unary `&` operator
  ///   or an integer constant cast to pointer type, or implicitly using an expression of array or function type.
  pub fn is_address_constant(&self) -> bool {
    match self.raw_expr() {
      RawExpr::Constant(c) => c.is_nullptr(),
      RawExpr::Unary(unary) if self.unqualified_type().is_pointer() =>
        unary.operand.is_lvalue()
          || matches!(unary.operand.unqualified_type(), Type::FunctionProto(_))
          || matches!(unary.operand.raw_expr(),
          RawExpr::Variable(var) if var.name.borrow().storage_class.is_static()),
      _ => false,
    }
  }

  /// 6.6.13: A structure or union constant is a named constant or compound literal constant with structure or union type, respectively.
  pub fn struct_or_union_constant(&self) -> bool {
    todo!()
  }

  /// 6.6.6
  pub fn compound_literal_constant(&self) -> bool {
    todo!()
  }
}

impl Expression {
  pub fn fold_unchecked(&mut self) -> Result<(), DiagData> {
    todo!()
  }
}
