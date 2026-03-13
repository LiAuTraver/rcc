use ::std::fmt::Display;

use super::{
  Module, Type, ValueData,
  instruction::{self as inst, Instruction},
  module, types,
  value::{Value, ValueID},
};

impl Display for Module<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.globals.iter().try_for_each(|(value_id)| {
      let global = &self.values[*value_id];
      match &global.value {
        Value::Function(function) => self.fmt_func(*value_id, global, f),
        Value::Variable(variable) => self.fmt_var(*value_id, global, f),
        Value::Instruction(_)
        | Value::Constant(_)
        | Value::BasicBlock(_)
        | Value::Argument(_) => todo!("not here!"),
      }
    })
  }
}

impl Module<'_> {
  fn fmt_func(
    &self,
    function_id: ValueID,
    function: &ValueData<'_>,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    let mut do_counter = 0_usize;
    let mut counter = || {
      let id = do_counter;
      do_counter += 1;
      id
    };
    let func = function.value.as_function_unchecked();
    write!(
      f,
      "{} {} @{}({})",
      if func.is_definition() {
        "define"
      } else {
        "declare"
      },
      function.ir_type.as_function_unchecked().return_type,
      func.name,
      func
        .params
        .iter()
        .map(|param_id| format!(
          "{} %{}",
          self.values[*param_id].ir_type,
          counter()
        ))
        .collect::<Vec<_>>()
        .join(", "),
    )?;
    if func.is_definition() {
      writeln!(f, "{{")?;
      write!(f, "{}", {
        let mut s = func
          .blocks
          .iter()
          .map(|block_id| {
            self.values[*block_id]
              .value
              .as_basicblock_unchecked()
              .instructions
              .iter()
              .map(|inst_id| {
                match &self.values[*inst_id].value.as_instruction_unchecked() {
                  Instruction::Call(call) => format!(
                    "%{} = call {} @{} ({})",
                    counter(),
                    self.values[call.callee]
                      .ir_type
                      .as_function_unchecked()
                      .return_type,
                    match &self.values[call.callee].value {
                      Value::Function(func) => func.name,
                      _ => todo!(),
                    },
                    call
                      .args
                      .iter()
                      .zip(
                        self.values[call.callee]
                          .ir_type
                          .as_function_unchecked()
                          .params
                          .iter()
                      )
                      .map(|(arg_id, param_type)| {
                        let arg = &self.values[*arg_id];
                        match &arg.value {
                          Value::Constant(constant) =>
                            format!("{} {}", param_type, constant),
                          _ => todo!(),
                        }
                      })
                      .collect::<Vec<_>>()
                      .join(", ")
                  ),
                  _ => todo!(),
                }
              })
              .collect::<Vec<_>>()
              .join("\n")
          })
          .collect::<Vec<_>>();
        s.push("<A terminator>".to_string());
        s.join("\n")
      })?;
      writeln!(f, "\n}}")
    } else {
      writeln!(f)
    }
  }

  fn fmt_var(
    &self,
    variable_id: ValueID,
    variable: &ValueData<'_>,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    writeln!(f, "not implemented");
    todo!()
  }
}

impl Display for types::Function<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}
impl Display for types::Array<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}
impl Display for types::Struct<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}

impl Display for Type<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Void => write!(f, "void"),
      Self::Label => write!(f, "label"),
      Self::Float => write!(f, "float"),
      Self::Double => write!(f, "double"),
      Self::Pointer => write!(f, "ptr"),
      Self::Integer(bit_width) => write!(f, "i{bit_width}"),
      Self::Array(array) => array.fmt(f),
      Self::Function(function) => function.fmt(f),
      Self::Struct(s) => s.fmt(f),
    }
  }
}
