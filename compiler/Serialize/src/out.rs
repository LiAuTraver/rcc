use ::termcolor::{BufferedStandardStream, ColorSpec, WriteColor};

use super::{FlushOnDropRAII, Palette, StickyWriter};

pub mod macros {
  #[macro_export]
  macro_rules! quoted {
    ($quote:literal => $value:expr) => {
      format_args!("{}{}{}", $quote, $value, $quote)
    };
    ($prefix:expr, $value:expr, $suffix:expr) => {
      format_args!("{}{}{}", $prefix, $value, $suffix)
    };
  }
  #[macro_export]
  macro_rules! pre {
    ($prefix:literal => $value:expr) => {
      format_args!("{}{}", $prefix, $value)
    };
  }
  #[macro_export]
  macro_rules! suff {
    ($suffix:literal => $value:expr) => {
      format_args!("{}{}", $value, $suffix)
    };
  }
}

pub struct Default<
  const INDENT_BODY: &'static str = "    ",
  const INDENT_LAST: &'static str = "    ",
  const PARENT_BODY: &'static str = "    ",
  const PARENT_LAST: &'static str = "    ",
  const PREFIX_LEFT: &'static str = "",
> {
  pub stream: StickyWriter<FlushOnDropRAII<BufferedStandardStream>>,
  pub palette: Palette,
}
impl<
  const INDENT_BODY: &'static str,
  const INDENT_LAST: &'static str,
  const PARENT_BODY: &'static str,
  const PARENT_LAST: &'static str,
  const PREFIX_LEFT: &'static str,
> Default<INDENT_BODY, INDENT_LAST, PARENT_BODY, PARENT_LAST, PREFIX_LEFT>
{
  #[inline]
  pub fn write_fmt(
    &mut self,
    args: ::std::fmt::Arguments<'_>,
    spec: &ColorSpec,
  ) {
    let _ = self.stream.set_color(spec);
    self.stream.write_fmt(args)
  }

  #[inline(always)]
  pub fn newline(&mut self) {
    writeln!(self.stream)
  }

  pub fn print_indent(&mut self, prefix: &str, is_last: bool) {
    let _ = self.stream.set_color(&self.palette.skeleton);
    write!(
      self.stream,
      "{}{}",
      prefix,
      if is_last { INDENT_LAST } else { INDENT_BODY }
    )
  }

  /// Build the new prefix for children based on whether the current node is the last child.
  #[inline]
  pub fn child_prefix(&self, prefix: &str, is_last: bool) -> String {
    format!(
      "{}{}",
      prefix,
      // parent was last → no vertical bar
      // parent was not last → vertical bar continues
      if is_last { PARENT_LAST } else { PARENT_BODY }
    )
  }

  #[inline(always)]
  pub fn palette(&self) -> &Palette {
    &self.palette
  }

  #[inline(always)]
  pub fn finalize(self) -> ::std::io::Result<()> {
    let mut stream = self.stream;
    stream.reset()?;
    stream.finalize()
  }

  pub fn new(
    stream: StickyWriter<FlushOnDropRAII<BufferedStandardStream>>,
    palette: Palette,
  ) -> Self {
    Self { stream, palette }
  }
}
