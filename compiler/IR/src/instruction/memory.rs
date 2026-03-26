use ::rcc_utils::static_dispatch;

use super::User;
use crate::ValueID;

/// Store value to address: *addr = value
///
/// [`Store::addr`] must have pointer type
#[derive(Debug)]
pub struct Store {
  operand: [ValueID; 2],
}

impl Store {
  pub fn new(target: ValueID, from: ValueID) -> Self {
    Self {
      operand: [target, from],
    }
  }

  pub fn dest(&self) -> ValueID {
    self.operand[0]
  }

  pub fn data(&self) -> ValueID {
    self.operand[1]
  }
}
impl User for Store {
  fn use_list(&self) -> &[ValueID] {
    &self.operand
  }
}

/// Load value from address: result = *addr
#[derive(Debug)]
pub struct Load {
  operand: [ValueID; 1],
}

impl Load {
  pub fn new(from: ValueID) -> Self {
    Self { operand: [from] }
  }

  pub fn addr(&self) -> ValueID {
    self.operand[0]
  }
}
impl User for Load {
  fn use_list(&self) -> &[ValueID] {
    &self.operand
  }
}
/// Stack allocation.
/// result = alloca typeof(type)
/// Used for local variables that must live in memory (e.g., if their address is taken).
#[derive(Debug, Default)]
pub struct Alloca;

impl Alloca {
  pub fn new() -> Self {
    Self
  }
}
impl User for Alloca {
  fn use_list(&self) -> &[ValueID] {
    &[]
  }
}
/// memory opeartion's `addr` must have type [`super::Type::Pointer`]
/// and the pointee type cannot be [`super::Type::Function`] or [`super::Type::Label`] (opaque pointer, we cannotr know, MUST check at construction),
/// which means the `Value` behind `ValueID` cannnot be a [`super::module::Function`] or [`super::BasicBlock`].
#[derive(Debug)]
pub enum Memory {
  Store(Store),
  Load(Load),
  Alloca(Alloca),
}
impl User for Memory {
  fn use_list(&self) -> &[ValueID] {
    static_dispatch!(self, |variant| variant.use_list() => Store Load Alloca)
  }
}
