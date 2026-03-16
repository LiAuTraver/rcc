#![allow(unused)]

use super::{Constant, Module, Value, ValueData, instruction as inst, module};
use crate::{
  common::{Dumpable, Dumper, FakeDumpRes, Palette, TreeDumper},
  ir,
};
// no tree structure for IR
pub type IRDumper<'source, 'context, 'session> = TreeDumper<
  'source,
  'context,
  'session,
  /* "    ", */
  /* "    ", */
  /* "    ", */
  /* "    ", */
  /* ""    , */
>;

impl<'context> Dumpable<'context> for Module {
  fn dump<'source, 'session>(
    &self,
    dumper: &mut impl Dumper<'source, 'context, 'session>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
  ) -> FakeDumpRes
  where
    'source: 'context,
    'context: 'session,
  {
    self.globals.iter().for_each(|value_id| {
      Dumpable::dump(
        &*ctx!(dumper).get(*value_id),
        dumper,
        prefix,
        is_last,
        palette,
      )
    })
  }
}

impl<'context> Dumpable<'context> for Value<'context> {
  fn dump<'source, 'session>(
    &self,
    dumper: &mut impl Dumper<'source, 'context, 'session>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
  ) -> FakeDumpRes
  where
    'source: 'context,
    'context: 'session,
  {
    ::rcc_utils::static_dispatch!(
        ValueData: &self.data,
        |variant| Dump::dump(self, dumper, prefix, is_last, palette, variant) =>
        Instruction Constant Function Variable BasicBlock Argument
    )
  }
}
trait Dump<'context, DataTy> {
  /// This is a special version of [`Dumpable::dump`] for dumping a specific variant of [`ValueData`].
  ///
  /// Please refer to the doc of [`Dumpable::dump`] for the meaning of the parameters.
  fn dump<'source, 'session>(
    &self,
    dumper: &mut impl Dumper<'source, 'context, 'session>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
    variant: &DataTy,
  ) -> FakeDumpRes
  where
    'source: 'context,
    'context: 'session;
}
/// Useless stuffs toi bypass type checker for now.
#[allow(unused)]
macro_rules! please_dump_me {
  ($DataTy:ty) => {
    #[allow(unused)]
    impl<'context> Dump<'context, $DataTy> for Value<'context> {
      fn dump<'source, 'session>(
        &self,
        dumper: &mut impl Dumper<'source, 'context, 'session>,
        prefix: &str,
        is_last: bool,
        palette: &Palette,
        variant: &$DataTy,
      ) -> FakeDumpRes
      where
        'source: 'context,
        'context: 'session,
      {
        todo!()
      }
    }
  };
}
impl<'context> Dump<'context, inst::Instruction> for Value<'context> {
  fn dump<'source, 'session>(
    &self,
    dumper: &mut impl Dumper<'source, 'context, 'session>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
    variant: &inst::Instruction,
  ) -> FakeDumpRes
  where
    'source: 'context,
    'context: 'session,
  {
    // my static_dispatch uses `ident` instead of
    // `type` of the 1st arg(qual path is unstable and rust-analyzer is having a hard time to hightlighing that).
    // hence strip the `::` path here.
    use inst::Instruction;
    ::rcc_utils::static_dispatch!(
        Instruction : variant,
        |variant| Dump::dump(self, dumper, prefix, is_last, palette, variant) =>
        Phi Terminator Unary Binary Memory Cast Call ICmp
    )
  }
}

impl<'context> Dump<'context, Constant<'_>> for Value<'context> {
  fn dump<'source, 'session>(
    &self,
    dumper: &mut impl Dumper<'source, 'context, 'session>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
    variant: &Constant<'_>,
  ) -> FakeDumpRes
  where
    'source: 'context,
    'context: 'session,
  {
    dumper.write_fmt(suff!(" " => self.ir_type), &palette.meta);
    dumper.write((variant), &palette.literal);
  }
}
impl<'context> Dump<'context, module::Function<'_>> for Value<'context> {
  fn dump<'source, 'session>(
    &self,
    dumper: &mut impl Dumper<'source, 'context, 'session>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
    variant: &module::Function<'_>,
  ) -> FakeDumpRes
  where
    'source: 'context,
    'context: 'session,
  {
    dumper.write_fmt(
      format_args!(
        "{} ",
        if variant.is_definition() {
          debug_assert!(
            variant.params.len()
              == self.ir_type.as_function_unchecked().params.len()
          );
          "define"
        } else {
          debug_assert!(
            variant.params.is_empty(),
            "my design ensures function decl has correct ir type, but the \
             argid is not stored."
          );
          "declare"
        }
      ),
      &palette.meta,
    );

    dumper.write_fmt(
      suff!(" " => self.ir_type.as_function_unchecked().return_type),
      &palette.meta,
    );

    dumper.write_fmt(format_args!("@{}(", variant.name), &palette.dim);
    if variant.is_definition() {
      variant
        .params
        .iter()
        .enumerate()
        .for_each(|(index, arg_id)| {
          let arg = &*ctx!(dumper).get(*arg_id);
          Dump::dump(
            arg,
            dumper,
            /* index */ &format!("{}", arg_id),
            index == variant.params.len() - 1,
            palette,
            arg.data.as_argument_unchecked(),
          );
        });
    } else {
      self
        .ir_type
        .as_function_unchecked()
        .params
        .iter()
        .enumerate()
        .for_each(|(index, param_ty)| {
          dumper.write_fmt(suff!(" " => param_ty), &palette.meta);
          dumper.write_fmt(
            format_args!(
              "{}",
              if variant.params.is_empty() || index + 1 == variant.params.len()
              {
                ""
              } else {
                ", "
              }
            ),
            &palette.dim,
          );
        });
    }
    dumper.write(")", &palette.dim);
    if variant.is_definition() {
      dumper.writeln(" {", &palette.meta);
      variant.blocks.iter().for_each(|block_id| {
        dumper.write_fmt(format_args!("{}:\n", block_id), &palette.dim);
        let block = &*ctx!(dumper).get(*block_id);
        Dump::dump(
          block,
          dumper,
          "\t",
          false,
          palette,
          block.data.as_basicblock_unchecked(),
        );
      });
      dumper.write("}", &palette.meta);
    }
    dumper.newline();
  }
}
impl<'context> Dump<'context, module::Variable<'_>> for Value<'context> {
  fn dump<'source, 'session>(
    &self,
    dumper: &mut impl Dumper<'source, 'context, 'session>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
    variant: &module::Variable<'_>,
  ) -> FakeDumpRes
  where
    'source: 'context,
    'context: 'session,
  {
    todo!()
  }
}
impl<'context> Dump<'context, module::BasicBlock> for Value<'context> {
  fn dump<'source, 'session>(
    &self,
    dumper: &mut impl Dumper<'source, 'context, 'session>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
    variant: &module::BasicBlock,
  ) -> FakeDumpRes
  where
    'source: 'context,
    'context: 'session,
  {
    variant.instructions.iter().for_each(|inst_id| {
      dumper.write(prefix, &palette.dim);
      let value = ctx!(dumper).get(*inst_id);
      match &value.data {
        ValueData::Instruction(inst::Instruction::Memory(
          inst::Memory::Store(s),
        )) => Dump::dump(&*value, dumper, "", is_last, palette, s),
        _ => {
          dumper.write_fmt(format_args!("%{} = ", inst_id), &palette.dim);
          Dumpable::dump(&*value, dumper, "", is_last, palette);
        },
      }
      dumper.newline();
    });
    let terminator = &*ctx!(dumper).get(variant.terminator);
    dumper.write(prefix, &palette.dim);
    Dump::dump(
      terminator,
      dumper,
      "",
      true,
      palette,
      terminator
        .data
        .as_instruction_unchecked()
        .as_terminator_unchecked(),
    );
  }
}
impl<'context> Dump<'context, module::Argument> for Value<'context> {
  fn dump<'source, 'session>(
    &self,
    dumper: &mut impl Dumper<'source, 'context, 'session>,
    index: &str, // this is actually an index
    is_last: bool,
    palette: &Palette,
    variant: &module::Argument,
  ) -> FakeDumpRes
  where
    'source: 'context,
    'context: 'session,
  {
    dumper.write_fmt(suff!(" " => self.ir_type), &palette.meta);
    dumper.write_fmt(pre!("%" => index), &palette.dim);
    dumper.write((if is_last { "" } else { ", " }), &palette.dim);
  }
}

please_dump_me!(inst::Phi);
impl<'context> Dump<'context, inst::Terminator> for Value<'context> {
  fn dump<'source, 'session>(
    &self,
    dumper: &mut impl Dumper<'source, 'context, 'session>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
    variant: &inst::Terminator,
  ) -> FakeDumpRes
  where
    'source: 'context,
    'context: 'session,
  {
    use inst::Terminator;
    ::rcc_utils::static_dispatch!(
        Terminator : variant,
        |variant| Dump::dump(self, dumper, prefix, is_last, palette, variant) =>
        Jump Branch Return
    )
  }
}
please_dump_me!(inst::Unary);
please_dump_me!(inst::Binary);
impl<'context> Dump<'context, inst::Memory> for Value<'context> {
  fn dump<'source, 'session>(
    &self,
    dumper: &mut impl Dumper<'source, 'context, 'session>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
    variant: &inst::Memory,
  ) -> FakeDumpRes
  where
    'source: 'context,
    'context: 'session,
  {
    use inst::Memory;
    ::rcc_utils::static_dispatch!(
        Memory : variant,
        |variant| Dump::dump(self, dumper, prefix, is_last, palette, variant) =>
        Alloca Load Store
    )
  }
}
please_dump_me!(inst::Cast);
impl<'context> Dump<'context, inst::Call> for Value<'context> {
  fn dump<'source, 'session>(
    &self,
    dumper: &mut impl Dumper<'source, 'context, 'session>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
    variant: &inst::Call,
  ) -> FakeDumpRes
  where
    'source: 'context,
    'context: 'session,
  {
    dumper.write("call ", &palette.literal);
    match &ctx!(dumper).get(variant.callee).data {
      ValueData::Instruction(instruction) => todo!(),
      ValueData::Constant(raw_constant) => todo!(),
      ValueData::Variable(variable) => todo!(),
      ValueData::Argument(argument) => todo!(),
      ValueData::Function(function) => {
        dumper.write_fmt(suff!(" " => self.ir_type), &palette.meta);
        dumper.write_fmt(quoted!(" @", function.name, "("), &palette.dim);
        variant.args.iter().enumerate().for_each(|(index, arg_id)| {
          let arg = &*ctx!(dumper).get(*arg_id);
          dumper.write_fmt(suff!(" " => arg.ir_type), &palette.meta);
          dumper.write_fmt(
            format_args!(
              "{}{}",
              arg.data.as_constant().map_or_else(
                || format!("%{}", arg_id),
                |constant| format!("{}", constant)
              ),
              if index == variant.args.len() - 1 {
                ""
              } else {
                ", "
              }
            ),
            &palette.dim,
          );
        });
        dumper.write(")", &palette.dim);
      },
      ValueData::BasicBlock(basic_block) => unreachable!(),
    }
  }
}
please_dump_me!(inst::ICmp);

please_dump_me!(inst::Jump);
please_dump_me!(inst::Branch);
impl<'context> Dump<'context, inst::Return> for Value<'context> {
  fn dump<'source, 'session>(
    &self,
    dumper: &mut impl Dumper<'source, 'context, 'session>,
    prefix: &str,
    _is_last: bool,
    palette: &Palette,
    variant: &inst::Return,
  ) -> FakeDumpRes
  where
    'source: 'context,
    'context: 'session,
  {
    debug_assert!(_is_last);
    dumper.write("ret ", &palette.literal);
    dumper.write_fmt(suff!(" " => self.ir_type), &palette.meta);
    if let Some(value_id) = variant.result {
      dumper.writeln_fmt(pre!("%" => value_id), &palette.dim);
    }
  }
}

impl<'context> Dump<'context, inst::Alloca> for Value<'context> {
  fn dump<'source, 'session>(
    &self,
    dumper: &mut impl Dumper<'source, 'context, 'session>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
    variant: &inst::Alloca,
  ) -> FakeDumpRes
  where
    'source: 'context,
    'context: 'session,
  {
    dumper.write("alloca ", &palette.literal);
    dumper.write(
      dumper.session().ir_context.ir_type(&self.qualified_type),
      &palette.meta,
    )
  }
}
impl<'context> Dump<'context, inst::Load> for Value<'context> {
  fn dump<'source, 'session>(
    &self,
    dumper: &mut impl Dumper<'source, 'context, 'session>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
    variant: &inst::Load,
  ) -> FakeDumpRes
  where
    'source: 'context,
    'context: 'session,
  {
    dumper.write("load ", &palette.literal);
    dumper.write((self.ir_type), &palette.meta);
    dumper.write(", ", &palette.skeleton);

    debug_assert!(ctx!(dumper).get(variant.addr).ir_type.is_pointer());

    dumper.write("ptr ", &palette.meta);
    dumper.write_fmt(pre!("@" => variant.addr), &palette.dim);
  }
}
impl<'context> Dump<'context, inst::Store> for Value<'context> {
  fn dump<'source, 'session>(
    &self,
    dumper: &mut impl Dumper<'source, 'context, 'session>,
    prefix: &str,
    _is_last: bool,
    palette: &Palette,
    variant: &inst::Store,
  ) -> FakeDumpRes
  where
    'source: 'context,
    'context: 'session,
  {
    dumper.write(prefix, &palette.dim);
    dumper.write("store ", &palette.literal);
    dumper.write(suff!(" " => self.ir_type), &palette.meta);
    // dumper.write(pre!("%" => variant.value), &palette.literal);
    // dumper.write(", ", &palette.skeleton);
    dumper.write_fmt(
      format_args!(
        "{}{}",
        ctx!(dumper)
          .get(variant.value)
          .data
          .as_constant()
          .map_or_else(
            || format!("%{}", variant.value),
            |constant| format!("{}", constant),
          ),
        ", "
      ),
      &palette.dim,
    );

    debug_assert!(ctx!(dumper).get(variant.addr).ir_type.is_pointer());

    dumper.write("ptr ", &palette.meta);
    dumper.write_fmt(pre!("%" => variant.addr), &palette.dim);
  }
}
