use super::User;
use crate::ValueID;

/// result = unary_op operand
#[derive(Debug)]
pub struct Unary {
  operator: UnaryOp,
  operand: [ValueID; 1],
}
#[derive(Debug)]
pub enum UnaryOp {
  FNeg,
}
impl Unary {
  pub fn new(operator: UnaryOp, operand: ValueID) -> Self {
    Self {
      operator,
      operand: [operand],
    }
  }

  pub fn operand(&self) -> ValueID {
    self.operand[0]
  }

  pub fn operator(&self) -> &UnaryOp {
    &self.operator
  }

  pub fn set_operand(&mut self, operand: ValueID) {
    self.operand[0] = operand;
  }
}

impl User for Unary {
  fn use_list(&self) -> &[ValueID] {
    &self.operand
  }
}

/// Function call: result = call func(args)
///
/// - [`Call::callee`] is usually a [`super::module::Function`], but can also be other except [`super::BasicBlock`].
/// - [`Call::args`] cannot contain [`super::BasicBlock`] and [`super::Function`] (always as a pointer form -- load ptr inst)
/// - The size of [`Call::args`] must match the parameter count of the parameter counts in [`super::types::Function`].
#[derive(Debug)]
pub struct Call {
  // pub callee: ValueID,
  // pub args: Vec<ValueID>,
  operands: Vec<ValueID>, // [callee, arg1, arg2, ...]
}

impl Call {
  pub fn new(operands: Vec<ValueID>) -> Self {
    Self { operands }
  }

  pub fn callee(&self) -> ValueID {
    self.operands[0]
  }

  pub fn args(&self) -> &[ValueID] {
    &self.operands[1..]
  }
}

impl User for Call {
  fn use_list(&self) -> &[ValueID] {
    &self.operands
  }
}
/// result = phi [val1, label1], [val2, label2]
///
/// if phi being used, it must be at the start of current block and has as many pairs as the branch had.
#[derive(Debug, Clone)]
pub struct Phi {
  operands: Vec<ValueID>, // (Value, From_Block_Label) pair.
}

impl Phi {
  pub fn new(operands: Vec<ValueID>) -> Self {
    Self { operands }
  }

  pub fn flat_view(&self) -> &[ValueID] {
    &self.operands
  }

  pub fn incomings(&self) -> &[(ValueID, ValueID)] {
    debug_assert!(self.operands.len().is_multiple_of(2));
    unsafe {
      ::std::slice::from_raw_parts(
        self.operands.as_ptr() as *const (ValueID, ValueID),
        self.operands.len() / 2,
      )
    }
  }
}
impl User for Phi {
  fn use_list(&self) -> &[ValueID] {
    &self.operands
  }
}

#[derive(Debug)]
pub struct Select {
  operands: [ValueID; 3], // [condition, true_value, false_value]
}
impl User for Select {
  fn use_list(&self) -> &[ValueID] {
    &self.operands
  }
}
impl Select {
  pub fn new(
    condition: ValueID,
    true_value: ValueID,
    false_value: ValueID,
  ) -> Self {
    Self {
      operands: [condition, true_value, false_value],
    }
  }

  pub fn condition(&self) -> ValueID {
    self.operands[0]
  }

  pub fn true_value(&self) -> ValueID {
    self.operands[1]
  }

  pub fn false_value(&self) -> ValueID {
    self.operands[2]
  }
}
#[derive(Debug)]
pub struct GetElementPtr {
  operands: Vec<ValueID>,
}

impl GetElementPtr {
  pub fn new(operands: Vec<ValueID>) -> Self {
    Self { operands }
  }

  pub fn base(&self) -> ValueID {
    self.operands[0]
  }

  pub fn indices(&self) -> &[ValueID] {
    &self.operands[1..]
  }
}
impl User for GetElementPtr {
  fn use_list(&self) -> &[ValueID] {
    &self.operands
  }
}
