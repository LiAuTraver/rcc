use ::std::{io::Write, mem::MaybeUninit};
use ::termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use crate::session::Session;

pub type DumpRes = ::std::io::Result<()>;

#[derive(Default, Clone)]
pub struct Palette {
  pub node_type: ColorSpec, // "BinaryExpr"
  pub operator: ColorSpec,  // "+"/"*"
  pub literal: ColorSpec,   // "42", "'a'"
  pub meta: ColorSpec,      // types, offsets
  pub dim: ColorSpec,       // span info, pointers
  pub error: ColorSpec,     // overflow info, error nodes
}
::rcc_utils::static_assert!(::std::mem::needs_drop::<Palette>() == false);
impl Palette {
  pub fn colored() -> Self {
    let mut node_type = ColorSpec::new();
    node_type.set_fg(Some(Color::Cyan)).set_bold(true);

    let mut operator = ColorSpec::new();
    operator.set_fg(Some(Color::Yellow));

    let mut literal = ColorSpec::new();
    literal.set_fg(Some(Color::Green));

    let mut meta = ColorSpec::new();
    meta.set_fg(Some(Color::Blue));

    let mut dim = ColorSpec::new();
    dim.set_fg(Some(Color::White)).set_intense(false); // let it be grey-ish

    let mut error = ColorSpec::new();
    error.set_fg(Some(Color::Red)).set_bold(true);

    Self {
      node_type,
      operator,
      literal,
      meta,
      dim,
      error,
    }
  }
}

pub trait Dumper {
  #[inline(always)]
  fn write(&mut self, text: &str, spec: &ColorSpec) -> DumpRes {
    self.write_fmt(format_args!("{}", text), spec)
  }
  #[inline(always)]
  fn writeln(&mut self, text: &str, spec: &ColorSpec) -> DumpRes {
    self.write_fmt(format_args!("{}\n", text), spec)
  }

  fn write_fmt(
    &mut self,
    args: ::std::fmt::Arguments<'_>,
    spec: &ColorSpec,
  ) -> DumpRes;

  fn newline(&mut self) -> DumpRes;
  fn print_indent(&mut self, prefix: &str, is_last: bool) -> DumpRes;

  fn dump(dumpable: &impl Dumpable) -> DumpRes;

  #[must_use]
  fn palette(&self) -> &Palette;
}
pub struct ASTDumper<'session> {
  pub(crate) stream: StandardStream,
  pub(crate) palette: Palette,
  /// currently no use, just keep maybe for future extensions.
  #[allow(dead_code)]
  pub(crate) session: &'session Session,
}
impl<'session> Dumper for ASTDumper<'session> {
  #[inline]
  fn write_fmt(
    &mut self,
    args: ::std::fmt::Arguments<'_>,
    spec: &ColorSpec,
  ) -> DumpRes {
    self.stream.set_color(spec)?;
    self.stream.write_fmt(args)?;
    self.stream.reset()
  }

  #[inline(always)]
  fn newline(&mut self) -> DumpRes {
    writeln!(self.stream)
  }

  fn print_indent(&mut self, prefix: &str, is_last: bool) -> DumpRes {
    let marker = if is_last { "└── " } else { "├── " };
    // Use the 'dim' style for the actual tree lines
    self.stream.set_color(&self.palette.dim)?;
    write!(self.stream, "{}{}", prefix, marker)?;
    self.stream.reset()
  }

  fn dump(dumpable: &impl Dumpable) -> DumpRes {
    let mut this = Self {
      stream: StandardStream::stdout(ColorChoice::Auto),
      palette: Palette::colored(),
      #[allow(clippy::uninit_assumed_init, invalid_value)]
      session: unsafe { MaybeUninit::uninit().assume_init() }, // won't actually be used for now.
    };
    let palette = this.palette.clone();
    dumpable.dump(&mut this, "", true, &palette)
  }

  #[inline(always)]
  fn palette(&self) -> &Palette {
    &self.palette
  }
}

impl<'session> ASTDumper<'session> {
  pub fn new(
    session: &'session Session,
    stream: StandardStream,
    palette: Palette,
  ) -> Self {
    Self {
      session,
      stream,
      palette,
    }
  }
}

pub trait Dumpable {
  /// Recurse through the tree.
  /// 'prefix' is the string of vertical bars from parents.
  /// 'is_last' determines if we use a `└──` or `├──`.
  ///
  /// Usually, the implementation should:
  /// 1. print the indent for **this** node. i.e., use [`Dumper::print_indent`] with the given `prefix` and `is_last`.
  /// 2. print the node header info like type name, address, span, etc. using [`Dumper::write_fmt`].
  /// 3. compute the prefix for children using static funtion [`Dumpable::child_prefix`] and recurse into children with the new `prefix` and correct `is_last`.
  fn dump(
    &self,
    dumper: &mut impl Dumper,
    prefix: &str,
    is_last: bool,
    palette: &Palette,
  ) -> DumpRes;
  /// Build the new prefix for children based on whether the current node is the last child.
  #[inline]
  fn child_prefix(prefix: &str, is_last: bool) -> String {
    if is_last {
      format!("{}    ", prefix) // parent was last → no vertical bar
    } else {
      format!("{}│   ", prefix) // parent was not last → vertical bar continues
    }
  }
}
