#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![feature(unsized_const_params)]

mod colored;
mod dumpable;
mod dumper;
mod out;
mod printable;
mod printer;
mod render;

pub use self::{
  colored::{FlushOnDropRAII, Palette, StickyWriter},
  dumper::{ASTDumper, DumpSpan, Dumpable, Dumper},
  out::Default as TreeDumper,
  printer::{IRPrinter, Printable},
  render::RenderEngine,
};

#[inline(never)]
pub fn render_ast<'c, D: ::rcc_shared::Diagnosis<'c>>(
  dumpable: &impl Dumpable<'c>,
  session: &'c ::rcc_ast::Session<'c, D>,
) -> ::std::io::Result<()> {
  ASTDumper::dump(dumpable, session)
}

#[inline(never)]
pub fn render_ir<'c, D: ::rcc_shared::Diagnosis<'c>>(
  printable: &'c impl Printable<'c>,
  session: &'c ::rcc_ir::Session<'c, D>,
) -> ::std::io::Result<()> {
  IRPrinter::print(printable, session)
}
