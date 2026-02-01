// use ::rc_utils::{DisplayWith, IntoWith};
// use ::thiserror::Error;

// use crate::{
//   common::{SourceManager, SourceSpan, Storage},
//   types::Qualifiers,
// };

// #[allow(dead_code)]
// type CustomMessage = String;
// type Elem = String;

// #[derive(Debug)]
// pub struct Warning {
//   pub span: SourceSpan,
//   pub data: Data,
// }
// #[derive(Debug, Error)]
// pub enum Data {

// }

// impl Warning {
//   pub fn new(span: SourceSpan, data: Data) -> Self {
//     Self { span, data }
//   }
// }
// impl IntoWith<SourceSpan, Warning> for Data {
//   fn into_with(self, span: SourceSpan) -> Warning {
//     Warning::new(span, self)
//   }
// }
// pub struct WarningDisplay<'a> {
//   warning: &'a Warning,
//   source_manager: &'a SourceManager,
// }
// impl<'a> DisplayWith<'a, SourceManager, WarningDisplay<'a>> for Warning {
//   fn display_with(
//     &'a self,
//     source_manager: &'a SourceManager,
//   ) -> WarningDisplay<'a> {
//     WarningDisplay {
//       warning: self,
//       source_manager,
//     }
//   }
// }
// impl<'a> ::std::fmt::Display for WarningDisplay<'a> {
//   fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
//     write!(
//       f,
//       "{}: {}",
//       self.warning.span.display_with(self.source_manager),
//       self.warning.data
//     )
//   }
// }
