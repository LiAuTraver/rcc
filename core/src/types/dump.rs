use super::{
  Array, Enum, FunctionProto, Pointer, Primitive, QualifiedType, Record, Type,
  Union,
};
use crate::common::{DumpRes, Dumpable, Dumper, Palette};

impl<'context> Dumpable for QualifiedType<'context> {
  fn dump<'t, 's>(
    &self,
    dumper: &mut impl Dumper<'t, 's>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
  ) -> DumpRes
  where
    's: 't,
  {
    dumper.print_indent(prefix, is_last)?;
    dumper.write("QualifiedType", &palette.node_type)?;
    dumper.write_fmt(format_args!(" {:p} ", self), &palette.dim)?;

    dumper.write_fmt(
      format_args!("{} {}\n", self.unqualified_type, self.qualifiers),
      &palette.meta,
    )?;

    let subprefix = dumper.child_prefix(prefix, is_last);
    self
      .unqualified_type
      .dump(dumper, &subprefix, true, palette)
  }
}

impl<'context> Dumpable for Type<'context> {
  fn dump<'t, 's>(
    &self,
    dumper: &mut impl Dumper<'t, 's>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
  ) -> DumpRes
  where
    's: 't,
  {
    ::rcc_utils::static_dispatch!(
      self.dump(dumper, prefix, is_last, palette),
      Primitive Pointer Array FunctionProto Union Enum Record
    )
  }
}
impl Dumpable for Primitive {
  fn dump<'t, 's>(
    &self,
    dumper: &mut impl Dumper<'t, 's>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
  ) -> DumpRes
  where
    's: 't,
  {
    dumper.print_indent(prefix, is_last)?;
    dumper.write("Primitive", &palette.node_type)?;
    dumper.write_fmt(format_args!(" {:p} ", self), &palette.dim)?;
    dumper.write_fmt(format_args!("{}\n", self), &palette.meta)
  }
}

impl<'context> Dumpable for Pointer<'context> {
  fn dump<'t, 's>(
    &self,
    dumper: &mut impl Dumper<'t, 's>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
  ) -> DumpRes
  where
    's: 't,
  {
    dumper.print_indent(prefix, is_last)?;
    dumper.write("Pointer", &palette.node_type)?;
    dumper.write_fmt(format_args!(" {:p} ", self), &palette.dim)?;
    dumper.write_fmt(format_args!("{}\n", self), &palette.meta)?;

    let subprefix = dumper.child_prefix(prefix, is_last);
    self.pointee.dump(dumper, &subprefix, true, palette)
  }
}
impl<'context> Dumpable for Array<'context> {
  fn dump<'t, 's>(
    &self,
    dumper: &mut impl Dumper<'t, 's>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
  ) -> DumpRes
  where
    's: 't,
  {
    dumper.print_indent(prefix, is_last)?;
    dumper.write("Array", &palette.node_type)?;
    dumper.write_fmt(format_args!(" {:p} ", self), &palette.dim)?;
    dumper.write_fmt(
      format_args!("{}, {} elements\n", self.element_type, self.size),
      &palette.meta,
    )?;

    let subprefix = dumper.child_prefix(prefix, is_last);
    self.element_type.dump(dumper, &subprefix, true, palette)
  }
}

impl<'context> Dumpable for FunctionProto<'context> {
  fn dump<'t, 's>(
    &self,
    dumper: &mut impl Dumper<'t, 's>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
  ) -> DumpRes
  where
    's: 't,
  {
    dumper.print_indent(prefix, is_last)?;
    dumper.write("FunctionProto", &palette.node_type)?;
    dumper.write_fmt(format_args!(" {:p} ", self), &palette.dim)?;
    dumper.write_fmt(format_args!("{}\n", self), &palette.meta)
  }
}
impl<'context> Dumpable for Enum<'context> {
  #[allow(unused)]
  fn dump<'t, 's>(
    &self,
    dumper: &mut impl Dumper<'t, 's>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
  ) -> DumpRes
  where
    's: 't,
  {
    todo!()
  }
}

impl<'context> Dumpable for Record<'context> {
  #[allow(unused)]
  fn dump<'t, 's>(
    &self,
    dumper: &mut impl Dumper<'t, 's>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
  ) -> DumpRes
  where
    's: 't,
  {
    todo!()
  }
}
impl<'context> Dumpable for Union<'context> {
  #[allow(unused)]
  fn dump<'t, 's>(
    &self,
    dumper: &mut impl Dumper<'t, 's>,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
  ) -> DumpRes
  where
    's: 't,
  {
    todo!()
  }
}
