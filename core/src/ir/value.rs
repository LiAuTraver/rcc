use super::{
  Argument, BasicBlock, Constant, TypeRef,
  instruction::Instruction,
  module::{Function, Variable},
};
use crate::types::QualifiedType;

::slotmap::new_key_type! {
    pub struct ValueID;
}

pub(super) trait Lookup<KeyType, ValueType> {
  fn lookup(&self, key: KeyType) -> &ValueType;
}

impl<'context> ValueID {
  pub(super) fn lookup(
    &self,
    arena: &'context impl Lookup<ValueID, ValueData<'context>>,
  ) -> &ValueData<'context> {
    arena.lookup(*self)
  }
}

#[derive(Debug)]
pub enum Value<'context> {
  Instruction(Instruction),
  Constant(Constant<'context>),
  Function(Function<'context>),
  Variable(Variable<'context>),
  BasicBlock(BasicBlock),
  Argument(Argument),
}

#[derive(Debug)]
pub struct ValueData<'context> {
  pub qualified_type: QualifiedType<'context>,
  pub ir_type: TypeRef<'context>,
  pub value: Value<'context>,
  pub users: Vec<ValueID>,
}

impl<'context> ValueData<'context> {
  pub fn new(
    qualified_type: QualifiedType<'context>,
    ir_type: TypeRef<'context>,
    value: Value<'context>,
  ) -> Self {
    Self {
      qualified_type,
      ir_type,
      value,
      users: Default::default(),
    }
  }
}

use ::rcc_utils::{interconvert, make_trio_for};
interconvert!(Instruction, Value<'context>);
interconvert!(Function, Value, 'context);
interconvert!(Constant, Value, 'context);
interconvert!(Variable, Value, 'context);
interconvert!(BasicBlock, Value<'context>);
interconvert!(Argument, Value<'context>);

make_trio_for!(Instruction, Value<'context>);
make_trio_for!(Function, Value, 'context);
make_trio_for!(Constant, Value, 'context);
make_trio_for!(Variable, Value, 'context);
make_trio_for!(BasicBlock, Value<'context>);
make_trio_for!(Argument, Value<'context>);
