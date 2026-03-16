use ::slotmap::Key;

use super::{Argument, Constant, Emitter, ValueID, instruction, module};
use crate::{
  ir::{Value, instruction::Instruction},
  types::QualifiedType,
};

/// Overload helper. I love overloading.
pub trait Emitable<'a, ValueType> {
  #[must_use]
  fn emit(
    &mut self,
    value: ValueType,
    qualified_type: QualifiedType<'a>,
  ) -> ValueID;
}

impl<'context> Emitable<'context, instruction::Terminator>
  for Emitter<'_, 'context, '_>
{
  fn emit(
    &mut self,
    terminator: instruction::Terminator,
    qualified_type: QualifiedType<'context>,
  ) -> ValueID {
    if let Some(block) = &mut self.current_block {
      assert!(block.terminator.is_null(), "block already has a terminator");
      let value_id = self.session.ir_context.insert(Value::new(
        qualified_type,
        ty!(self, qualified_type),
        Into::<Instruction>::into(terminator).into(),
      ));
      block.terminator = value_id;
      value_id
    } else {
      panic!("no block to emit terminator into")
    }
  }
}
impl<'context> Emitable<'context, instruction::Alloca>
  for Emitter<'_, 'context, '_>
{
  fn emit(
    &mut self,
    alloca: instruction::Alloca,
    qualified_type: QualifiedType<'context>,
  ) -> ValueID {
    if let Some(block) = &mut self.current_block {
      let value_id = self.session.ir_context.insert(Value::new(
        qualified_type,
        self.session.ir_context.pointer_type(),
        Into::<Instruction>::into(Into::<instruction::Memory>::into(alloca))
          .into(),
      ));

      block.instructions.push(value_id);
      value_id
    } else {
      panic!("no block to emit terminator into")
    }
  }
}
impl<'context, InstType: Into<Instruction>> Emitable<'context, InstType>
  for Emitter<'_, 'context, '_>
{
  default fn emit(
    &mut self,
    value: InstType,
    qualified_type: QualifiedType<'context>,
  ) -> ValueID {
    if let Some(block) = &mut self.current_block {
      let value_id = self.session.ir_context.insert(Value::new(
        qualified_type,
        ty!(self, qualified_type),
        value.into().into(),
      ));
      block.instructions.push(value_id);
      value_id
    } else {
      panic!("no block to emit into")
    }
  }
}

impl<'context> Emitable<'context, module::Function<'context>>
  for Emitter<'_, 'context, '_>
{
  fn emit(
    &mut self,
    value: module::Function<'context>,
    qualified_type: QualifiedType<'context>,
  ) -> ValueID {
    let value_id = self.session.ir_context.insert(Value::new(
      qualified_type,
      ty!(self, qualified_type),
      value.into(),
    ));
    self.module.globals.push(value_id);
    value_id
  }
}
impl<'context> Emitable<'context, module::Variable<'context>>
  for Emitter<'_, 'context, '_>
{
  fn emit(
    &mut self,
    value: module::Variable<'context>,
    qualified_type: QualifiedType<'context>,
  ) -> ValueID {
    let value_id = self.session.ir_context.insert(Value::new(
      qualified_type,
      ty!(self, qualified_type),
      value.into(),
    ));
    self.module.globals.push(value_id);
    value_id
  }
}
impl<'context> Emitable<'context, Constant<'context>>
  for Emitter<'_, 'context, '_>
{
  fn emit(
    &mut self,
    value: Constant<'context>,
    qualified_type: QualifiedType<'context>,
  ) -> ValueID {
    self.session.ir_context.insert(Value::new(
      qualified_type,
      ty!(self, qualified_type),
      value.into(),
    ))
  }
}
impl<'context> Emitable<'context, Argument> for Emitter<'_, 'context, '_> {
  fn emit(
    &mut self,
    value: Argument,
    qualified_type: QualifiedType<'context>,
  ) -> ValueID {
    self.session.ir_context.insert(Value::new(
      qualified_type,
      ty!(self, qualified_type),
      value.into(),
    ))
  }
}
