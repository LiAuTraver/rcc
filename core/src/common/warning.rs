use ::rc_utils::DisplayWith;

use super::{SourceManager, SourceSpan, Storage};
use crate::types::Qualifiers;

#[derive(Debug)]
pub struct Warning {
  pub span: SourceSpan,
  pub data: Data,
}
#[derive(Debug)]
pub enum Data {
  UnusedVariable(String),
  DeprecatedFunction(String),
  RedundantStorageSpecs(Storage),
  RedundantQualifier(Qualifiers),
  EmptyTypedef,
  EmptyStatement,
}

impl Warning {
  pub fn new(span: SourceSpan, data: Data) -> Self {
    Self { span, data }
  }
}
pub struct WarningDisplay<'a> {
  warning: &'a Warning,
  source_manager: &'a SourceManager,
}
impl<'a> DisplayWith<'a, SourceManager, WarningDisplay<'a>> for Warning {
  fn display_with(
    &'a self,
    source_manager: &'a SourceManager,
  ) -> WarningDisplay<'a> {
    WarningDisplay {
      warning: self,
      source_manager,
    }
  }
}
impl<'a> ::std::fmt::Display for WarningDisplay<'a> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(
      f,
      "{}: ",
      self.warning.span.display_with(self.source_manager)
    )?;

    match &self.warning.data {
      Data::UnusedVariable(name) => write!(f, "Unused variable '{}'", name),
      Data::DeprecatedFunction(name) =>
        write!(f, "Deprecated function '{}'", name),
      Data::EmptyStatement => write!(f, "Empty statement"),
      Data::RedundantStorageSpecs(storage) =>
        write!(f, "Redundant storage specifiers '{storage}'"),
      Data::RedundantQualifier(qualifiers) =>
        write!(f, "Redundant type qualifiers '{qualifiers}'"),
      Data::EmptyTypedef => write!(f, "Typedef defines nothing"),
    }
  }
}
