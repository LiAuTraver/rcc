use ::rc_utils::DisplayWith;

use super::{Operator, SourceManager, SourceSpan, Storage};

/// im focusing the ast right now, so left error handling as a placeholder
pub type Error = ();
/// Custom message. would be printed as-is.
type CustomMessage = String;
/// Element, like `expect ')' after <elem>`
type Elem = String;
/// Error `Version 2`. Will replace the old `Error` type (which is just ()) soon.
#[derive(Debug)]
pub struct ErrorV2 {
  pub span: SourceSpan,
  pub data: Data,
}
#[derive(Debug)]
pub enum Data {
  // lexing errors
  UnexpectedCharacter(char),
  UnterminatedString,
  InvalidNumberFormat(String),
  // parseing errors
  MissingOperator(Operator),
  MultipleStorageSpecs(Storage, Storage),
  MissingTypeSpecifier(CustomMessage),
  MissingIdentifier(CustomMessage),
  ExtraneousComma(CustomMessage),
  VoidVariableDecl(CustomMessage),
  ExtraneousStorageSpecs(Storage),
  UnclosedParameterList(CustomMessage),
  MissingOpenParen(Elem),
  MissingCloseParen(Elem),
  ExpressionNotConstant(CustomMessage),
  VarDeclUnclosed(CustomMessage),
  InvalidBlockItem,
  MissingFunctionName,
  InvalidStmt(CustomMessage),
  CaseLabelAfterDefault,
  MultipleDefaultLabels,
  MissingLabelInSwitch,
  CaseLabelNotWithinSwitch,
  DefaultLabelNotWithinSwitch,
  TopLevelLabel,
  MissingLabelAfterGoto,
  InvalidBreakStmt,
  InvalidContinueStmt,
  // placeholder for future errors
  Placeholder(String),
}
impl ErrorV2 {
  pub fn new(span: SourceSpan, data: Data) -> Self {
    Self { span, data }
  }
}
pub struct ErrorDisplay<'a> {
  error: &'a ErrorV2,
  source_manager: &'a SourceManager,
}

impl<'a> DisplayWith<'a, SourceManager, ErrorDisplay<'a>> for ErrorV2 {
  fn display_with(
    &'a self,
    source_manager: &'a SourceManager,
  ) -> ErrorDisplay<'a> {
    ErrorDisplay {
      error: self,
      source_manager,
    }
  }
}
impl<'a> ::std::fmt::Display for ErrorDisplay<'a> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}: ", self.error.span.display_with(self.source_manager))?;

    match &self.error.data {
      Data::UnexpectedCharacter(c) => write!(f, "Unexpected character '{}'", c),
      Data::UnterminatedString => write!(f, "Unterminated string literal"),
      Data::InvalidNumberFormat(s) =>
        write!(f, "Invalid number format '{}'", s),
      Data::MissingOperator(operator) => write!(f, "Expect '{}'", operator),
      Data::MultipleStorageSpecs(storage1, storage2) => write!(
        f,
        "Cannot combine storage classes '{}' and '{}'",
        storage1, storage2
      ),
      Data::MissingTypeSpecifier(_) => write!(
        f,
        "Expect a type specifier in declaration, default to 'int'"
      ),
      Data::MissingIdentifier(_) =>
        write!(f, "Expect identifier in declarator"),
      Data::ExtraneousComma(msg) => write!(f, "{msg}"),
      Data::VoidVariableDecl(msg) => write!(f, "{msg}"),
      Data::ExtraneousStorageSpecs(storage) =>
        write!(f, "Storage class specifier '{storage}' is not allowed here"),
      Data::UnclosedParameterList(msg) => write!(f, "{msg}"),
      Data::MissingOpenParen(msg) => write!(f, "Expect '(' after {msg}"),
      Data::MissingCloseParen(msg) => write!(f, "Expect ')' after {msg}"),
      Data::ExpressionNotConstant(msg) =>
        write!(f, "Expression '{msg}' is not a constant"),
      Data::VarDeclUnclosed(msg) => write!(f, "{msg}"),
      Data::InvalidBlockItem =>
        write!(f, "Block definition is not allowed here"),
      Data::MissingFunctionName => write!(f, "Expect function name"),
      Data::InvalidStmt(msg) => write!(f, "{msg}"),
      Data::CaseLabelAfterDefault =>
        write!(f, "Case label cannot appear after default label"),
      Data::MultipleDefaultLabels => write!(
        f,
        "Multiple default labels in one switchl ignoring the latter"
      ),
      Data::MissingLabelInSwitch =>
        write!(f, "Expect at least one case or default label in switch"),
      Data::CaseLabelNotWithinSwitch =>
        write!(f, "Case label not within switch"),
      Data::DefaultLabelNotWithinSwitch =>
        write!(f, "Default label not within switch"),
      Data::TopLevelLabel => write!(f, "Label cannot appear at top level"),
      Data::MissingLabelAfterGoto =>
        write!(f, "Expect label identifier after goto"),
      Data::InvalidBreakStmt =>
        write!(f, "Break statement not within loop or switch"),
      Data::InvalidContinueStmt =>
        write!(f, "Continue statement not within loop"),
      Data::Placeholder(_) => todo!(),
    }
  }
}
