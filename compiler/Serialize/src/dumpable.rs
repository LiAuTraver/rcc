use ::rcc_ast::types::{
  Array, Enum, FunctionProto, Pointer, Primitive, QualifiedType, Record, Type,
  Union,
};
use ::std::fmt;

use crate::{DumpSpan, Dumpable, Dumper, Palette};

impl<'c> Dumpable<'c> for QualifiedType<'_> {
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
  ) {
    dumper.print_indent(prefix, is_last);
    dumper.write("QualifiedType", &palette.node);
    dumper.write_fmt(format_args!(" {:p} ", self), &palette.dim);

    dumper.write_fmt(
      format_args!("{} {}\n", self.unqualified_type, self.qualifiers),
      &palette.meta,
    );

    let subprefix = dumper.child_prefix(prefix, is_last);
    self
      .unqualified_type
      .dump(dumper, &subprefix, true, palette)
  }
}

impl<'c> Dumpable<'c> for Type<'_> {
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
  ) {
    ::rcc_utils::static_dispatch!(
      self,
      |variant| variant.dump(dumper, prefix, is_last, palette) =>
      Primitive Pointer Array FunctionProto Union Enum Record
    )
  }
}
impl<'c> Dumpable<'c> for Primitive {
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
  ) {
    dumper.print_indent(prefix, is_last);
    dumper.write("Primitive", &palette.node);
    dumper.write_fmt(format_args!(" {:p} ", self), &palette.dim);
    dumper.write_fmt(format_args!("{}\n", self), &palette.meta)
  }
}

impl<'c> Dumpable<'c> for Pointer<'_> {
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
  ) {
    dumper.print_indent(prefix, is_last);
    dumper.write("Pointer", &palette.node);
    dumper.write_fmt(format_args!(" {:p} ", self), &palette.dim);
    dumper.write_fmt(format_args!("{}\n", self), &palette.meta);

    let subprefix = dumper.child_prefix(prefix, is_last);
    self.pointee.dump(dumper, &subprefix, true, palette)
  }
}
impl<'c> Dumpable<'c> for Array<'_> {
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
  ) {
    dumper.print_indent(prefix, is_last);
    dumper.write("Array", &palette.node);
    dumper.write_fmt(format_args!(" {:p} ", self), &palette.dim);
    dumper.write_fmt(
      format_args!("{}, {} elements\n", self.element_type, self.size),
      &palette.meta,
    );

    let subprefix = dumper.child_prefix(prefix, is_last);
    self.element_type.dump(dumper, &subprefix, true, palette)
  }
}

impl<'c> Dumpable<'c> for FunctionProto<'_> {
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
  ) {
    dumper.print_indent(prefix, is_last);
    dumper.write("FunctionProto", &palette.node);
    dumper.write_fmt(format_args!(" {:p} ", self), &palette.dim);
    dumper.write_fmt(format_args!("{}\n", self), &palette.meta)
  }
}
#[allow(unused)]
impl<'c> Dumpable<'c> for Enum<'_> {
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
  ) {
    todo!()
  }
}

#[allow(unused)]
impl<'c> Dumpable<'c> for Record<'_> {
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
  ) {
    todo!()
  }
}
#[allow(unused)]
impl<'c> Dumpable<'c> for Union<'_> {
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
  ) {
    todo!()
  }
}

mod statement {
  use ::rcc_ast::blueprints;
  pub type Empty = blueprints::Placeholder;
  pub type Return<'c, E> = blueprints::RawReturn<E>;
  pub type If<'c, S, E> = blueprints::RawIf<S, E>;
  pub type While<'c, S, E> = blueprints::RawWhile<S, E>;
  pub type DoWhile<'c, S, E> = blueprints::RawDoWhile<S, E>;
  pub type For<'c, S, E> = blueprints::RawFor<S, E>;
  pub type Switch<'c, S, E, C> = blueprints::RawSwitch<S, E, C>;
  pub type Case<'c, S, C> = blueprints::RawCase<S, C>;
  pub type Default<'c, S> = blueprints::RawDefault<S>;
  pub type Label<'c, S> = blueprints::RawLabel<'c, S>;
  pub type Goto<'c> = blueprints::RawGoto<'c>;
  pub type Compound<'c, S> = blueprints::RawCompound<S>;
  pub type Break<'c> = blueprints::RawBreak;
  pub type Continue<'c> = blueprints::RawContinue;
}

use statement::*;

impl<'c> Dumpable<'c> for statement::Empty {
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
  ) {
    dumper.print_indent(prefix, is_last);
    dumper.write("EmptyStmt", &palette.node);
    dumper.write_fmt(format_args!(" {:p}\n", self), &palette.dim)
  }
}

macro_rules! headers {
  (
    $self:ident,
    $dumper:ident,
    $prefix:ident,
    $is_last:ident,
    $palette:ident,
    $name:expr
  ) => {{
    $dumper.print_indent($prefix, $is_last);
    $dumper.write($name, &$palette.node);
    $dumper.write_fmt(format_args!(" {:p} ", $self), &$palette.dim);
    $self.span.dump($dumper, $prefix, $is_last, &$palette);
    $dumper.newline()
  }};
}

impl<'c, E> Dumpable<'c> for Return<'_, E>
where
  E: Dumpable<'c>,
{
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
  ) {
    headers!(self, dumper, prefix, is_last, palette, "Return");

    if let Some(expr) = &self.expression {
      let subprefix = dumper.child_prefix(prefix, is_last);
      expr.dump(dumper, &subprefix, true, palette);
    }
  }
}

impl<'c, S> Dumpable<'c> for Compound<'_, S>
where
  S: Dumpable<'c>,
{
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
  ) {
    headers!(self, dumper, prefix, is_last, palette, "Compound");

    let subprefix = dumper.child_prefix(prefix, is_last);
    self.statements.iter().enumerate().for_each(|(i, stmt)| {
      stmt.dump(dumper, &subprefix, i == self.statements.len() - 1, palette)
    })
  }
}

impl<'c, S, E> Dumpable<'c> for If<'_, S, E>
where
  S: Dumpable<'c>,
  E: Dumpable<'c>,
{
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
  ) {
    headers!(self, dumper, prefix, is_last, palette, "If");

    let subprefix = dumper.child_prefix(prefix, is_last);
    self.condition.dump(dumper, &subprefix, false, palette);
    self.then_branch.dump(
      dumper,
      &subprefix,
      self.else_branch.is_none(),
      palette,
    );
    if let Some(else_branch) = &self.else_branch {
      else_branch.dump(dumper, &subprefix, true, palette);
    }
  }
}

impl<'c, S, E> Dumpable<'c> for While<'_, S, E>
where
  S: Dumpable<'c>,
  E: Dumpable<'c>,
{
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
  ) {
    headers!(self, dumper, prefix, is_last, palette, "While");

    let subprefix = dumper.child_prefix(prefix, is_last);
    self.condition.dump(dumper, &subprefix, false, palette);
    self.body.dump(dumper, &subprefix, true, palette)
  }
}

impl<'c, S, E> Dumpable<'c> for DoWhile<'_, S, E>
where
  S: Dumpable<'c>,
  E: Dumpable<'c>,
{
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
  ) {
    headers!(self, dumper, prefix, is_last, palette, "DoWhile");

    let subprefix = dumper.child_prefix(prefix, is_last);
    self.body.dump(dumper, &subprefix, false, palette);
    self.condition.dump(dumper, &subprefix, true, palette)
  }
}

impl<'c, S, E> Dumpable<'c> for For<'_, S, E>
where
  S: Dumpable<'c>,
  E: Dumpable<'c>,
{
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
  ) {
    headers!(self, dumper, prefix, is_last, palette, "For");

    let subprefix = dumper.child_prefix(prefix, is_last);
    if let Some(init) = &self.initializer {
      init.dump(dumper, &subprefix, false, palette);
    }
    if let Some(cond) = &self.condition {
      cond.dump(dumper, &subprefix, false, palette);
    }
    if let Some(incr) = &self.increment {
      incr.dump(dumper, &subprefix, false, palette);
    }
    self.body.dump(dumper, &subprefix, true, palette)
  }
}

impl<'c, S, E, C> Dumpable<'c> for Switch<'_, S, E, C>
where
  S: Dumpable<'c> + fmt::Display,
  E: Dumpable<'c> + fmt::Display,
  C: fmt::Display,
{
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
  ) {
    headers!(self, dumper, prefix, is_last, palette, "Switch");

    let subprefix = dumper.child_prefix(prefix, is_last);
    self.condition.dump(dumper, &subprefix, false, palette);
    self.cases.iter().enumerate().for_each(|(i, case)| {
      case.dump(
        dumper,
        &subprefix,
        (i == self.cases.len() - 1) && self.default.is_none(),
        palette,
      )
    });
    if let Some(default) = &self.default {
      default.dump(dumper, &subprefix, true, palette);
    }
  }
}
impl<'c, S, E> Dumpable<'c> for Case<'_, S, E>
where
  S: Dumpable<'c> + fmt::Display,
  E: fmt::Display,
{
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
  ) {
    headers!(self, dumper, prefix, is_last, palette, "Case");

    let subprefix = dumper.child_prefix(prefix, is_last);
    dumper.write_fmt(format_args!("Value: {}\n", self.value), &palette.literal);
    self.body.iter().enumerate().for_each(|(i, stmt)| {
      stmt.dump(dumper, &subprefix, i == self.body.len() - 1, palette)
    })
  }
}
impl<'c, S> Dumpable<'c> for statement::Default<'_, S>
where
  S: Dumpable<'c>,
{
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
  ) {
    headers!(self, dumper, prefix, is_last, palette, "Default");

    let subprefix = dumper.child_prefix(prefix, is_last);
    self.body.iter().enumerate().for_each(|(i, stmt)| {
      stmt.dump(dumper, &subprefix, i == self.body.len() - 1, palette)
    })
  }
}

impl<'c> Dumpable<'c> for Goto<'_> {
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
  ) {
    dumper.print_indent(prefix, is_last);
    dumper.write("Goto", &palette.node);
    dumper.write_fmt(format_args!(" {:p} ", self), &palette.dim);
    self.span.dump(dumper, prefix, is_last, palette);
    dumper.write_fmt(format_args!("'{}'", self.label), &palette.literal);
    dumper.newline()
  }
}

impl<'c, S> Dumpable<'c> for Label<'_, S>
where
  S: Dumpable<'c>,
{
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
  ) {
    dumper.print_indent(prefix, is_last);
    dumper.write("Label", &palette.node);

    dumper.write_fmt(format_args!(" {:p} ", self), &palette.dim);
    self.span.dump(dumper, prefix, is_last, palette);
    dumper.write_fmt(format_args!(" '{}'\n", self.name), &palette.literal);

    let subprefix = dumper.child_prefix(prefix, is_last);
    self.statement.dump(dumper, &subprefix, true, palette);
  }
}

impl<'c> Dumpable<'c> for Break<'_> {
  #[inline]
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
  ) {
    headers!(self, dumper, prefix, is_last, palette, "Break")
  }
}

impl<'c> Dumpable<'c> for Continue<'_> {
  #[inline]
  fn dump(
    &self,
    dumper: &mut impl Dumper<'c>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
  ) {
    headers!(self, dumper, prefix, is_last, palette, "Continue")
  }
}
