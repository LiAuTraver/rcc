#![allow(unused)]

use ::rcc_utils::DisplayWith;

use super::{
  Context, Module, Value, ValueData, ValueID, instruction as inst, module,
};
use crate::{
  common::{Dumpable, Dumper, FakeDumpRes, Integral, Palette, TreeDumper},
  ir,
  types::Constant,
};
// no tree structure for IR
pub type IRDumper<'c> = TreeDumper<
  'c,
  /* "    ", */
  /* "    ", */
  /* "    ", */
  /* "    ", */
  /* ""    , */
>;

use ::std::cell::RefCell;

thread_local! {
  /// just a workaround and ill redo it later.
  static COUNTER: RefCell<Vec<u64>> = const { RefCell::new(Vec::new()) };
}

// if find the handle, return the index, otherwise push it and return the new index.
pub(super) fn counter(value_id: ValueID) -> usize {
  COUNTER.with(|c| {
    let mut vec = c.borrow_mut();
    if let Some(index) = vec.iter().position(|&h| h == value_id.handle()) {
      index
    } else {
      vec.push(value_id.handle());
      vec.len() - 1
    }
  })
}
fn pretty_dump_contant_or_id<'a>(
  dumper: &mut impl Dumper<'a>,
  value_id: ValueID,
  palette: &Palette,
  ir_type: bool,
) {
  if ir_type {
    dumper.write(
      suff!(" " => dumper.session().ir().get(value_id).ir_type),
      &palette.meta,
    );
  }
  if let Some(value) = dumper.session().ir().get_by_constant_id(&value_id) {
    match dumper.session().ir().get(value_id).ir_type {
      ir::Type::Floating(_) => dumper.write_fmt(
        format_args!("{:.e}", value.as_floating_unchecked()),
        &palette.literal,
      ),
      ir::Type::Pointer() => {
        debug_assert!(value.is_nullptr());
        dumper.write("null", &palette.literal)
      },
      ir::Type::Integer(1u8) => dumper.write(value.is_one(), &palette.literal),
      // if the value is max, dump it as -1 for better readability.
      ir::Type::Integer(width) => match value.as_integral_unchecked() {
        bitmask if *bitmask == Integral::bitmask(*width) =>
          dumper.write("-1", &palette.literal),
        integer => dumper.write(integer, &palette.literal),
      },
      _ => dumper.write(value, &palette.literal),
    }
  } else {
    dumper.write(pre!("%"=> counter(value_id)), &palette.skeleton)
  }
}
fn print_users<'a>(
  dumper: &mut impl Dumper<'a>,
  palette: &Palette,
  value_id: ValueID,
) -> FakeDumpRes {
  let print_current_id = dumper.session().ir().get(value_id).ir_type.is_void();
  let use_list = dumper.session().ir().get_use_list(value_id);
  let usees = || {
    use_list
      .iter()
      .map(|&user_id| format!("%{}", counter(user_id)))
      .collect::<Vec<_>>()
      .join(", ")
  };
  let args = match (print_current_id, use_list.is_empty()) {
    (true, true) => format_args!("\t\t\t\t; id %{}", counter(value_id)),
    (true, false) =>
      format_args!("\t\t\t\t; id %{}, used by {}", counter(value_id), usees()),
    (false, true) => format_args!(""),
    (false, false) => format_args!("\t\t\t\t; used by {}", usees()),
  };
  dumper.write_fmt(args, &palette.info);
}
impl<'c> Dumpable<'c> for Module {
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
  ) -> FakeDumpRes {
    self.globals.iter().for_each(|&value_id| {
      Dumpable::dump(
        &*lookup!(dumper, value_id),
        dumper,
        prefix,
        is_last,
        palette,
      )
    })
  }
}

impl<'c> Dumpable<'c> for Value<'c> {
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
  ) -> FakeDumpRes {
    ::rcc_utils::static_dispatch!(
        ValueData: &self.data,
        |variant| Dump::dump(self, dumper, prefix, is_last, palette, variant) =>
        Instruction Constant Function Variable BasicBlock Argument
    )
  }
}
trait Dump<'c, DataTy> {
  /// This is a special version of [`Dumpable::dump`] for dumping a specific variant of [`ValueData`].
  ///
  /// Please refer to the doc of [`Dumpable::dump`] for the meaning of the parameters.
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
    variant: &DataTy,
  ) -> FakeDumpRes;
}
/// Useless stuffs toi bypass type checker for now.
#[allow(unused)]
macro_rules! please_dump_me {
  ($DataTy:ty) => {
    #[allow(unused)]
    impl<'c> Dump<'c, $DataTy> for Value<'c> {
      fn dump(
        &self,
        dumper: &mut impl Dumper<'c>,
        prefix: &str,
        is_last: bool,
        palette: &Palette,
        variant: &$DataTy,
      ) -> FakeDumpRes {
        todo!()
      }
    }
  };
}
impl<'c> Dump<'c, inst::Instruction> for Value<'c> {
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
    variant: &inst::Instruction,
  ) -> FakeDumpRes {
    // my static_dispatch uses `ident` instead of
    // `type` of the 1st arg(qual path is unstable and rust-analyzer is having a hard time to hightlighing that).
    // hence strip the `::` path here.
    use inst::Instruction;
    ::rcc_utils::static_dispatch!(
        Instruction : variant,
        |variant| Dump::dump(self, dumper, prefix, is_last, palette, variant) =>
        Phi Terminator Unary Binary Memory Cast Call Cmp
    )
  }
}

impl<'c> Dump<'c, Constant<'_>> for Value<'c> {
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
    variant: &Constant<'_>,
  ) -> FakeDumpRes {
    dumper.write(suff!(" " => self.ir_type), &palette.meta);
    dumper.write(variant, &palette.literal);
  }
}
impl<'c> Dump<'c, module::Function<'_>> for Value<'c> {
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
    variant: &module::Function<'_>,
  ) -> FakeDumpRes {
    fn preds<'c>(
      dumper: &mut impl Dumper<'c>,
      palette: &Palette,
      block_id: ValueID,
    ) {
      let use_list = dumper.session().ir().get_use_list(block_id);
      if !use_list.is_empty() {
        dumper.write_fmt(
          format_args!(
            "\t\t\t\t\t; preds = {}",
            use_list
              .iter()
              .map(|&user_id| format!(
                "%{}",
                counter(lookup!(dumper, user_id).parent)
              ))
              .collect::<Vec<_>>()
              .join(", ")
          ),
          &palette.info,
        );
      }
    }

    dumper.write(
      suff!(
        " " =>
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
      &palette.literal,
    );

    dumper.write(
      suff!(" " => self.ir_type.as_function_unchecked().return_type),
      &palette.meta,
    );

    dumper.write(pre!("@" => variant.name), &palette.skeleton);
    dumper.write("(", &palette.skeleton);
    if variant.is_definition() {
      variant
        .params
        .iter()
        .enumerate()
        .for_each(|(index, &arg_id)| {
          let arg = &*lookup!(dumper, arg_id);
          Dump::dump(
            arg,
            dumper,
            /* index */ &format!("{}", counter(arg_id)),
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
          dumper.write(suff!(" " => param_ty), &palette.meta);
          dumper.write(
            if variant.params.is_empty() || index + 1 == variant.params.len() {
              ""
            } else {
              ", "
            },
            &palette.dim,
          );
        });
    }
    dumper.write(")", &palette.skeleton);
    if variant.is_definition() {
      dumper.writeln(" {", &palette.skeleton);
      variant.blocks.iter().for_each(|&block_id| {
        dumper.write(suff!(":" => counter(block_id)), &palette.skeleton);
        let block = &*lookup!(dumper, block_id);
        preds(dumper, palette, block_id);
        dumper.newline();
        Dump::dump(
          block,
          dumper,
          "\t",
          false,
          palette,
          block.data.as_basicblock_unchecked(),
        );
      });
      dumper.write("\n}", &palette.skeleton);
    }
    dumper.newline();

    COUNTER.with(|c| c.borrow_mut().clear());
  }
}
impl<'c> Dump<'c, module::Variable<'_>> for Value<'c> {
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
    variant: &module::Variable<'_>,
  ) -> FakeDumpRes {
    todo!()
  }
}
impl<'c> Dump<'c, module::BasicBlock> for Value<'c> {
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
    variant: &module::BasicBlock,
  ) -> FakeDumpRes {
    variant.instructions.iter().for_each(|&inst_id| {
      dumper.write(prefix, &palette.dim);
      let value = lookup!(dumper, inst_id);
      match &value.data {
        ValueData::Instruction(inst::Instruction::Memory(
          inst::Memory::Store(store),
        )) => {
          Dump::dump(&*value, dumper, "", is_last, palette, store);
          print_users(dumper, palette, inst_id);
        },
        _ => {
          dumper.write_fmt(pre!("%"=> counter(inst_id)), &palette.skeleton);
          dumper.write(" = ", &palette.skeleton);
          Dumpable::dump(&*value, dumper, "", is_last, palette);
          print_users(dumper, palette, inst_id);
        },
      }
      dumper.newline();
    });
    let terminator = &*lookup!(dumper, variant.terminator);
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
    print_users(dumper, palette, variant.terminator);
    dumper.newline();
  }
}
impl<'c> Dump<'c, module::Argument> for Value<'c> {
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    index: &str, // coontext is actually an index
    is_last: bool,
    palette: &Palette,
    variant: &module::Argument,
  ) -> FakeDumpRes {
    dumper.write(suff!(" " => self.ir_type), &palette.meta);
    dumper.write(pre!("%" => index), &palette.skeleton);
    dumper.write((if is_last { "" } else { ", " }), &palette.dim);
  }
}

please_dump_me!(inst::Phi);
impl<'c> Dump<'c, inst::Terminator> for Value<'c> {
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
    variant: &inst::Terminator,
  ) -> FakeDumpRes {
    use inst::Terminator;
    ::rcc_utils::static_dispatch!(
        Terminator : variant,
        |variant| Dump::dump(self, dumper, prefix, is_last, palette, variant) =>
        Jump Branch Return
    )
  }
}
please_dump_me!(inst::Unary);
impl<'c> Dump<'c, inst::Binary> for Value<'c> {
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
    variant: &inst::Binary,
  ) -> FakeDumpRes {
    dumper.write(suff!(" " => variant.operator()), &palette.literal);

    self::pretty_dump_contant_or_id(dumper, variant.left(), palette, true);
    dumper.write(", ", &palette.skeleton);
    self::pretty_dump_contant_or_id(dumper, variant.right(), palette, false);
  }
}
impl<'c> Dump<'c, inst::Memory> for Value<'c> {
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
    variant: &inst::Memory,
  ) -> FakeDumpRes {
    use inst::Memory;
    ::rcc_utils::static_dispatch!(
        Memory : variant,
        |variant| Dump::dump(self, dumper, prefix, is_last, palette, variant) =>
        Alloca Load Store
    )
  }
}
impl<'c> Dump<'c, inst::Cast> for Value<'c> {
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
    variant: &inst::Cast,
  ) -> FakeDumpRes {
    use inst::Cast;
    ::rcc_utils::static_dispatch!(
        Cast : variant,
        |variant| Dump::dump(self, dumper, prefix, is_last, palette, variant) =>
        Zext Sext Trunc
    )
  }
}
impl<'c> Dump<'c, inst::Call> for Value<'c> {
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
    variant: &inst::Call,
  ) -> FakeDumpRes {
    dumper.write("call ", &palette.literal);
    match &lookup!(dumper, variant.callee()).data {
      ValueData::Instruction(instruction) => todo!(),
      ValueData::Constant(constant) => todo!(),
      ValueData::Variable(variable) => todo!(),
      ValueData::Argument(argument) =>
        unreachable!("this should be impossible, or not implemented."),
      ValueData::Function(function) => {
        dumper.write(suff!(" " => self.ir_type), &palette.meta);
        dumper.write(quoted!(" @", function.name, "("), &palette.skeleton);
        variant
          .args()
          .iter()
          .enumerate()
          .for_each(|(index, &arg_id)| {
            let arg = &*lookup!(dumper, arg_id);
            dumper.write(suff!(" " => arg.ir_type), &palette.meta);
            dumper.write(
              arg.data.as_constant().map_or_else(
                || format!("%{}", counter(arg_id)),
                |constant| format!("{}", constant),
              ),
              &palette.skeleton,
            );
            dumper.write(", ", &palette.skeleton);
          });
        dumper.write(")", &palette.skeleton);
      },
      ValueData::BasicBlock(basic_block) => unreachable!(),
    }
  }
}
impl<'c> Dump<'c, inst::Cmp> for Value<'c> {
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
    variant: &inst::Cmp,
  ) -> FakeDumpRes {
    use inst::Cmp;
    ::rcc_utils::static_dispatch!(
        Cmp : variant,
        |variant| Dump::dump(self, dumper, prefix, is_last, palette, variant) =>
        ICmp FCmp
    )
  }
}
impl<'c> Dump<'c, inst::ICmp> for Value<'c> {
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
    variant: &inst::ICmp,
  ) -> FakeDumpRes {
    dumper.write("icmp ", &palette.literal);
    dumper.write(suff!(" " => variant.predicate), &palette.literal);

    self::pretty_dump_contant_or_id(dumper, variant.lhs(), palette, true);
    dumper.write(", ", &palette.skeleton);
    self::pretty_dump_contant_or_id(dumper, variant.rhs(), palette, false);
  }
}

impl<'c> Dump<'c, inst::FCmp> for Value<'c> {
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
    variant: &inst::FCmp,
  ) -> FakeDumpRes {
    dumper.write("fcmp ", &palette.literal);
    dumper.write(suff!(" " => variant.predicate), &palette.literal);

    self::pretty_dump_contant_or_id(dumper, variant.lhs(), palette, true);
    dumper.write(", ", &palette.skeleton);
    self::pretty_dump_contant_or_id(dumper, variant.rhs(), palette, false);
  }
}

impl<'c> Dump<'c, inst::Jump> for Value<'c> {
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
    variant: &inst::Jump,
  ) -> FakeDumpRes {
    dumper.write("br ", &palette.literal);
    debug_assert!(lookup!(dumper, variant.target()).ir_type.is_label());
    dumper.write("label ", &palette.meta);
    dumper.write(pre!("%" => counter(variant.target())), &palette.skeleton);
  }
}
impl<'c> Dump<'c, inst::Branch> for Value<'c> {
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
    variant: &inst::Branch,
  ) -> FakeDumpRes {
    dumper.write("br ", &palette.literal);
    // debug_assert!(self.ir_type.as_integer().is_some_and(|i| *i == 1u8));
    debug_assert!(self.ir_type.is_void());
    dumper.write("i1 ", &palette.meta);
    dumper.write(pre!("%" => counter(variant.condition())), &palette.skeleton);
    dumper.write(", ", &palette.skeleton);
    debug_assert!(lookup!(dumper, variant.then_branch()).ir_type.is_label());
    dumper.write("label ", &palette.meta);
    dumper.write(
      pre!("%" => counter(variant.then_branch())),
      &palette.skeleton,
    );

    dumper.write(", ", &palette.skeleton);
    debug_assert!(lookup!(dumper, variant.else_branch()).ir_type.is_label());
    dumper.write("label ", &palette.meta);
    dumper.write(
      pre!("%" => counter(variant.else_branch())),
      &palette.skeleton,
    );
  }
}
impl<'c> Dump<'c, inst::Return> for Value<'c> {
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    _is_last: bool,
    palette: &Palette,
    variant: &inst::Return,
  ) -> FakeDumpRes {
    debug_assert!(_is_last);
    dumper.write("ret ", &palette.literal);
    if let Some(value_id) = variant.result() {
      self::pretty_dump_contant_or_id(dumper, value_id, palette, true);
    }
  }
}

impl<'c> Dump<'c, inst::Alloca> for Value<'c> {
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
    variant: &inst::Alloca,
  ) -> FakeDumpRes {
    dumper.write("alloca ", &palette.literal);
    dumper.write(
      dumper.session().ir().ir_type(&self.qualified_type),
      &palette.meta,
    )
  }
}
impl<'c> Dump<'c, inst::Load> for Value<'c> {
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
    variant: &inst::Load,
  ) -> FakeDumpRes {
    dumper.write("load ", &palette.literal);
    dumper.write((self.ir_type), &palette.meta);
    dumper.write(", ", &palette.skeleton);

    debug_assert!(lookup!(dumper, variant.from()).ir_type.is_pointer());

    dumper.write("ptr ", &palette.meta);
    dumper.write(pre!("%" => counter(variant.from())), &palette.skeleton);
  }
}
impl<'c> Dump<'c, inst::Store> for Value<'c> {
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    _is_last: bool,
    palette: &Palette,
    variant: &inst::Store,
  ) -> FakeDumpRes {
    dumper.write(prefix, &palette.dim);
    dumper.write("store ", &palette.literal);

    self::pretty_dump_contant_or_id(dumper, variant.from(), palette, true);

    dumper.write(", ", &palette.skeleton);

    debug_assert!(lookup!(dumper, variant.target()).ir_type.is_pointer());

    dumper.write("ptr ", &palette.meta);
    dumper.write(pre!("%" => counter(variant.target())), &palette.skeleton);
  }
}

impl<'c> Dump<'c, inst::Zext> for Value<'c> {
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
    variant: &inst::Zext,
  ) -> FakeDumpRes {
    dumper.write("zext ", &palette.literal);

    self::pretty_dump_contant_or_id(dumper, variant.operand(), palette, true);

    dumper.write(" to ", &palette.skeleton);
    dumper.write(self.ir_type, &palette.meta);
  }
}
please_dump_me!(inst::Sext);
please_dump_me!(inst::Trunc);
