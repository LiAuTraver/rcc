use super::{SourceManager, SourceSpan};
use crate::common::SourceDisplay;

/// im focusing the ast right now, so left error handling as a placeholder
pub type Error = ();
/// Error `Version 2`. Will replace the old `Error` type (which is just ()) soon.
#[derive(Debug)]
pub struct ErrorV2 {
  pub span: SourceSpan,
  pub data: Data,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Data {
  // lexing errors
  UnexpectedCharacter(char),
  UnterminatedString,
  InvalidNumberFormat(String),
}
impl ErrorV2 {
  pub fn new(span: SourceSpan, data: Data) -> Self {
    Self { span, data }
  }
}
impl<'a> SourceDisplay<'a> for ErrorV2 {
  type ReturnType = ErrorDisplay<'a>;

  fn display_with(
    &'a self,
    source_manager: &'a SourceManager,
  ) -> Self::ReturnType {
    ErrorDisplay {
      error: self,
      source_manager,
    }
  }
}

pub struct ErrorDisplay<'a> {
  error: &'a ErrorV2,
  source_manager: &'a SourceManager,
}

impl<'a> ::std::fmt::Display for ErrorDisplay<'a> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}: ", self.error.span.display_with(self.source_manager))?;

    match &self.error.data {
      Data::UnexpectedCharacter(c) => write!(f, "Unexpected character '{}'", c),
      Data::UnterminatedString => write!(f, "Unterminated string literal"),
      Data::InvalidNumberFormat(s) =>
        write!(f, "Invalid number format '{}'", s),
    }
  }
}

pub struct SpanDisplay<'a> {
  span: &'a SourceSpan,
  source_manager: &'a SourceManager,
}
impl<'a> SourceDisplay<'a> for SourceSpan {
  type ReturnType = SpanDisplay<'a>;

  fn display_with(
    &'a self,
    source_manager: &'a SourceManager,
  ) -> Self::ReturnType {
    SpanDisplay {
      span: self,
      source_manager,
    }
  }
}
impl<'a> ::std::fmt::Display for SpanDisplay<'a> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let span = self.span;
    let file = &self.source_manager.files[span.file_index as usize];
    let coord = self.source_manager.lookup_line_col(*span);

    write!(
      f,
      "{}:{}:{}",
      file.path.to_str().unwrap_or("<invalid utf8>"),
      coord.line,
      coord.column
    )
  }
}
