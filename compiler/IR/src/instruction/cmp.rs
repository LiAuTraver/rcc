use ::rcc_adt::Signedness;
use ::rcc_shared::Operator;
use ::rcc_utils::static_dispatch;

use super::User;
use crate::ValueID;

#[derive(Debug)]
pub struct ICmp {
  predicate: ICmpPredicate,
  operand: [ValueID; 2],
}

impl ICmp {
  pub fn new(predicate: ICmpPredicate, lhs: ValueID, rhs: ValueID) -> Self {
    Self {
      predicate,
      operand: [lhs, rhs],
    }
  }

  pub fn predicate(&self) -> ICmpPredicate {
    self.predicate
  }

  pub fn lhs(&self) -> ValueID {
    self.operand[0]
  }

  pub fn rhs(&self) -> ValueID {
    self.operand[1]
  }
}
impl User for ICmp {
  fn use_list(&self) -> &[ValueID] {
    &self.operand
  }
}
#[derive(Debug, Clone, Copy, ::strum_macros::Display)]
#[strum(serialize_all = "lowercase")]
pub enum ICmpPredicate {
  Eq,
  Ne,
  Slt,
  Sle,
  Sgt,
  Sge,
  Ult,
  Ule,
  Ugt,
  Uge,
}
impl ICmpPredicate {
  pub const fn from_op_and_sign(
    operator: Operator,
    signedness: Signedness,
  ) -> Self {
    use ICmpPredicate::*;
    use Operator::*;
    use Signedness::*;
    match (operator, signedness) {
      (Less, Signed) => Slt,
      (Less, Unsigned) => Ult,
      (LessEqual, Signed) => Sle,
      (LessEqual, Unsigned) => Ule,
      (Greater, Signed) => Sgt,
      (Greater, Unsigned) => Ugt,
      (GreaterEqual, Signed) => Sge,
      (GreaterEqual, Unsigned) => Uge,
      (EqualEqual, _) => Eq,
      (NotEqual, _) => Ne,
      _ => unreachable!(),
    }
  }
}

#[derive(Debug)]
pub struct FCmp {
  predicate: FCmpPredicate,
  // pub lhs: ValueID,
  // pub rhs: ValueID,
  operand: [ValueID; 2],
}

impl FCmp {
  pub fn new(predicate: FCmpPredicate, lhs: ValueID, rhs: ValueID) -> Self {
    Self {
      predicate,
      operand: [lhs, rhs],
    }
  }

  pub fn predicate(&self) -> FCmpPredicate {
    self.predicate
  }

  pub fn lhs(&self) -> ValueID {
    self.operand[0]
  }

  pub fn rhs(&self) -> ValueID {
    self.operand[1]
  }
}
impl User for FCmp {
  fn use_list(&self) -> &[ValueID] {
    &self.operand
  }
}
#[derive(Debug, Clone, Copy, ::strum_macros::Display)]
#[strum(serialize_all = "lowercase")]
pub enum FCmpPredicate {
  /// Always `false` if `NaN` is involved.
  Oeq,
  One,
  Olt,
  Ole,
  Ogt,
  Oge,
  /// Always `true` if `NaN` is involved.
  Ueq,
  Une,
  Ult,
  Ule,
  Ugt,
  Uge,
}
impl FCmpPredicate {
  pub const fn from_op(operator: Operator) -> Self {
    use FCmpPredicate::*;
    use Operator::*;
    match operator {
      Less => Olt,
      LessEqual => Ole,
      Greater => Ogt,
      GreaterEqual => Oge,
      EqualEqual => Oeq,
      // `NaN` always not equal than other, even both are `NaN`.
      NotEqual => Une,
      _ => unreachable!(),
    }
  }
}
#[derive(Debug)]
pub enum Cmp {
  ICmp(ICmp),
  FCmp(FCmp),
}
impl User for Cmp {
  fn use_list(&self) -> &[ValueID] {
    static_dispatch!(self, |variant| variant.use_list() => ICmp FCmp)
  }
}
