use ::rcc_adt::Signedness;
use ::rcc_shared::Operator;

use super::User;
use crate::ValueID;

/// result = binary_op lhs, rhs
///
/// - The type of `lhs` and `rhs` must be the same.
/// - `lhs` and `rhs` cannot be [`super::module::Function`], [`super::module::BasicBlock`] or [`super::module::Variable`].  
#[derive(Debug)]
pub struct Binary {
  operator: BinaryOp,
  operand: [ValueID; 2],
}

impl Binary {
  pub fn new(operator: BinaryOp, left: ValueID, right: ValueID) -> Self {
    Self {
      operator,
      operand: [left, right],
    }
  }

  pub fn operator(&self) -> BinaryOp {
    self.operator
  }

  pub fn left(&self) -> ValueID {
    self.operand[0]
  }

  pub fn right(&self) -> ValueID {
    self.operand[1]
  }
}
impl User for Binary {
  fn use_list(&self) -> &[ValueID] {
    &self.operand
  }
}
// arithematic ops only consider integer for now
#[derive(Debug, Clone, Copy, ::strum_macros::Display)]
#[strum(serialize_all = "lowercase")]
pub enum BinaryOp {
  Add,
  FAdd,
  Sub,
  FSub,
  Mul,
  FMul,
  UDiv,
  SDiv,
  FDiv,
  URem,
  SRem,
  FRem,
  /// Bitwise And.
  And,
  /// Bitwise Or.
  Or,
  /// Bitwise eXclusive or.
  Xor,
  /// Shift Left.
  Shl,
  /// Logical Shift Right for unsigned integers.
  LShr,
  /// for signed integers.
  AShr,
}
impl BinaryOp {
  pub const fn from_op_and_sign(
    operator: Operator,
    signedness: Signedness,
    is_floating: bool,
  ) -> BinaryOp {
    use BinaryOp::*;
    use Operator::*;
    use Signedness::*;
    match (operator, is_floating, signedness) {
      (Plus, false, ..) => Add,
      (Plus, true, ..) => FAdd,
      (Minus, false, ..) => Sub,
      (Minus, true, ..) => FSub,
      (Star, false, ..) => Mul,
      (Star, true, ..) => FMul,
      (Slash, false, Signed) => SDiv,
      (Slash, false, Unsigned) => UDiv,
      (Slash, true, ..) => FDiv,
      (Percent, false, Signed) => SRem,
      (Percent, false, Unsigned) => URem,
      (Percent, true, ..) => FRem,
      (Ampersand, ..) => BinaryOp::And,
      (Pipe, ..) => BinaryOp::Or,
      (Caret, false, ..) => Xor,
      (LeftShift, false, ..) => Shl,
      (RightShift, false, Signed) => AShr,
      (RightShift, false, Unsigned) => LShr,
      _ => panic!("semantic analysis should catch this."),
    }
  }
}
