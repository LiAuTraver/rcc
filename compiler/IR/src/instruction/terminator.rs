use ::rcc_utils::static_dispatch;

use super::User;
use crate::ValueID;

/// Creater must ensure [`Jump::label`] must be am ID points to a [`super::BasicBlock`].
#[derive(Debug)]
pub struct Jump {
  operands: [ValueID; 1],
}

impl Jump {
  pub fn new(to: ValueID) -> Self {
    Self { operands: [to] }
  }

  pub fn target(&self) -> ValueID {
    self.operands[0]
  }

  pub fn set_target(&mut self, to: ValueID) {
    self.operands[0] = to;
  }
}
impl User for Jump {
  fn use_list(&self) -> &[ValueID] {
    &self.operands
  }
}
/// Creater must ensure [`Branch::true_label`] and [`Branch::false_label`] must be am ID points to a [`super::BasicBlock`].
///
/// The owner of this instruction must ensure the type of [`Branch::cond`] is i1 (boolean).
#[derive(Debug)]
pub struct Branch {
  operands: [ValueID; 3], // [cond, then_label, else_label]
}

impl Branch {
  pub fn new(
    condition: ValueID,
    then_branch: ValueID,
    else_branch: ValueID,
  ) -> Self {
    Self {
      operands: [condition, then_branch, else_branch],
    }
  }

  pub fn condition(&self) -> ValueID {
    self.operands[0]
  }

  pub fn then_branch(&self) -> ValueID {
    self.operands[1]
  }

  pub fn else_branch(&self) -> ValueID {
    self.operands[2]
  }

  pub fn set_condition(&mut self, condition: ValueID) {
    self.operands[0] = condition;
  }

  pub fn set_then_branch(&mut self, then_branch: ValueID) {
    self.operands[1] = then_branch;
  }

  pub fn set_else_branch(&mut self, else_branch: ValueID) {
    self.operands[2] = else_branch;
  }
}
impl User for Branch {
  fn use_list(&self) -> &[ValueID] {
    &self.operands
  }
}
/// Must match the return type of the function. For void function, [`Return::result`] should be [`None`].
#[derive(Debug)]
pub struct Return {
  operands: [ValueID; 1], // for void function, this operand should be null
}

impl Return {
  pub fn new(result: Option<ValueID>) -> Self {
    Self {
      operands: [result.unwrap_or(ValueID::null())],
    }
  }

  pub fn result(&self) -> Option<ValueID> {
    if self.operands[0].is_null() {
      None
    } else {
      Some(self.operands[0])
    }
  }

  pub fn set_result(&mut self, result: Option<ValueID>) {
    self.operands[0] = result.unwrap_or(ValueID::null());
  }
}
impl User for Return {
  fn use_list(&self) -> &[ValueID] {
    if self.operands[0].is_null() {
      &[]
    } else {
      &self.operands
    }
  }
}
#[derive(Debug, Default)]
pub struct Unreachable;

impl Unreachable {
  pub fn new() -> Self {
    Self
  }
}
impl User for Unreachable {
  fn use_list(&self) -> &[ValueID] {
    &[]
  }
}
#[derive(Debug)]
pub enum Terminator {
  /// Unconditional jump
  Jump(Jump),
  /// Conditional branch: if cond goto true_label else goto false_label
  Branch(Branch),
  /// Return from function
  Return(Return),
  /// Placeholder or unreachable.
  Unreachable(Unreachable),
}

impl User for Terminator {
  fn use_list(&self) -> &[ValueID] {
    static_dispatch!(self, |variant| variant.use_list() => Jump Branch Return Unreachable)
  }
}
