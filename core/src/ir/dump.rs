use super::{
  Constant, Context, Module, Value, ValueData, instruction as inst, module,
};
use crate::common::{DumpRes, Dumpable, Dumper, Palette, TreeDumper};
// no tree structure for IR
pub type IRDumper<'source, 'context, 'session> = TreeDumper<
  'session,
  'context,
  'source,
  /* "    ", */
  /* "    ", */
  /* "    ", */
  /* "    ", */
  /* ""    , */
>;

macro_rules! ctx {
  ($this:expr) => {
    $this.session().ir_context
  };
}

impl Dumpable for Module {
  fn dump<'source, 'context, 'session>(
    &self,
    dumper: &mut impl Dumper<'source, 'context, 'session>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
  ) -> DumpRes
  where
    'source: 'context,
    'context: 'session,
  {
    self.globals.iter().try_for_each(|value_id| {
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

impl Dumpable for ValueData<'_> {
  fn dump<'source, 'context, 'session>(
    &self,
    dumper: &mut impl Dumper<'source, 'context, 'session>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
  ) -> DumpRes
  where
    'source: 'context,
    'context: 'session,
  {
    ::rcc_utils::static_dispatch!(
        Value: &self.value,
        |variant| Dump::dump(self, dumper, prefix, is_last, palette, variant) =>
        Instruction Constant Function Variable BasicBlock Argument
    )
  }
}
trait Dump<ValueTy> {
  /// This is a special version of [`Dumpable::dump`] for dumping a specific variant of [`ValueData`].
  ///
  /// Please refer to the doc of [`Dumpable::dump`] for the meaning of the parameters.
  fn dump<'source, 'context, 'session>(
    &self,
    dumper: &mut impl Dumper<'source, 'context, 'session>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
    variant: &ValueTy,
  ) -> DumpRes
  where
    'source: 'context,
    'context: 'session;
}
/// Useless stuffs toi bypass type checker for now.
#[allow(unused)]
macro_rules! please_dump_me {
  ($ValueTy:ty) => {
    impl Dump<$ValueTy> for ValueData<'_> {
      fn dump<'source, 'context, 'session>(
        &self,
        dumper: &mut impl Dumper<'source, 'context, 'session>,
        prefix: &str,
        is_last: bool,
        palette: &Palette,
        variant: &$ValueTy,
      ) -> DumpRes
      where
        'source: 'context,
        'context: 'session,
      {
        todo!()
      }
    }
  };
}
impl Dump<inst::Instruction> for ValueData<'_> {
  fn dump<'source, 'context, 'session>(
    &self,
    dumper: &mut impl Dumper<'source, 'context, 'session>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
    variant: &inst::Instruction,
  ) -> DumpRes
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

impl Dump<Constant<'_>> for ValueData<'_> {
  fn dump<'source, 'context, 'session>(
    &self,
    dumper: &mut impl Dumper<'source, 'context, 'session>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
    variant: &Constant<'_>,
  ) -> DumpRes
  where
    'source: 'context,
    'context: 'session,
  {
    todo!()
  }
}
impl Dump<module::Function<'_>> for ValueData<'_> {
  fn dump<'source, 'context, 'session>(
    &self,
    dumper: &mut impl Dumper<'source, 'context, 'session>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
    variant: &module::Function<'_>,
  ) -> DumpRes
  where
    'source: 'context,
    'context: 'session,
  {
    todo!()
  }
}
impl Dump<module::Variable<'_>> for ValueData<'_> {
  fn dump<'source, 'context, 'session>(
    &self,
    dumper: &mut impl Dumper<'source, 'context, 'session>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
    variant: &module::Variable<'_>,
  ) -> DumpRes
  where
    'source: 'context,
    'context: 'session,
  {
    todo!()
  }
}
impl Dump<module::BasicBlock> for ValueData<'_> {
  fn dump<'source, 'context, 'session>(
    &self,
    dumper: &mut impl Dumper<'source, 'context, 'session>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
    variant: &module::BasicBlock,
  ) -> DumpRes
  where
    'source: 'context,
    'context: 'session,
  {
    todo!()
  }
}
impl Dump<module::Argument> for ValueData<'_> {
  fn dump<'source, 'context, 'session>(
    &self,
    dumper: &mut impl Dumper<'source, 'context, 'session>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
    variant: &module::Argument,
  ) -> DumpRes
  where
    'source: 'context,
    'context: 'session,
  {
    todo!()
  }
}

please_dump_me!(inst::Phi);
impl Dump<inst::Terminator> for ValueData<'_> {
  fn dump<'source, 'context, 'session>(
    &self,
    dumper: &mut impl Dumper<'source, 'context, 'session>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
    variant: &inst::Terminator,
  ) -> DumpRes
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
please_dump_me!(inst::Memory);
please_dump_me!(inst::Cast);
please_dump_me!(inst::Call);
please_dump_me!(inst::ICmp);

please_dump_me!(inst::Jump);
please_dump_me!(inst::Branch);
impl Dump<inst::Return> for ValueData<'_> {
  fn dump<'source, 'context, 'session>(
    &self,
    dumper: &mut impl Dumper<'source, 'context, 'session>,
    prefix: &str,
    _is_last: bool,
    palette: &Palette,
    variant: &inst::Return,
  ) -> DumpRes
  where
    'source: 'context,
    'context: 'session,
  {
    debug_assert!(_is_last);
    dumper.print_indent(prefix, true)?;
    dumper.write("ret ", &palette.literal)?;
    dumper.write_fmt(format_args!("{} ", self.ir_type), &palette.meta)?;
    dumper.write_fmt(
      format_args!(
        "{}",
        if let Some(value_id) = variant.result {
          todo!()
        } else {
          ""
        },
      ),
      &palette.kind,
    )?;
    dumper.newline()
  }
}
