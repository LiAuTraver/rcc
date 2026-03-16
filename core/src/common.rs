#[macro_use]
mod dumper;
mod environment;
mod floating;
mod integral;
mod keyword;
mod operator;
mod source_info;
mod storage;
mod token;
pub use self::{
  dumper::{Default as TreeDumper, Dumpable, Dumper, FakeDumpRes, Palette},
  environment::{Environment, Symbol, SymbolRef, UnitScope, VarDeclKind},
  floating::{Floating, Format as FloatFormat},
  integral::{Integral, Signedness},
  keyword::Keyword,
  operator::{Category as OperatorCategory, Operator},
  source_info::{
    Coordinate, Display as SourceDisplay, File as SourceFile,
    Id as SourceFileId, Id as FileId, Index as SourceSpanIndex,
    Index as SpanIndex, Manager as SourceManager, Span as SourceSpan,
    SpanDisplay,
  },
  storage::Storage,
  token::{Literal, Token},
};

pub type StrRef<'context> = &'context str;

pub trait RefEq {
  fn ref_eq(lhs: Self, rhs: Self) -> bool
  where
    Self: PartialEq + Sized;
}

impl<'a> RefEq for StrRef<'a> {
  fn ref_eq(lhs: Self, rhs: Self) -> bool
  where
    Self: PartialEq,
  {
    if cfg!(debug_assertions) && !::std::ptr::eq(lhs, rhs) && lhs == rhs {
      eprintln!(
        "INTERNAL INVARIANT: comparing types by pointer but they are actually \
         the same: {:p}: {:?} and {:p}: {:?}.",
        lhs, lhs, rhs, rhs
      );
      return true;
    }
    ::std::ptr::eq(lhs, rhs)
  }
}
