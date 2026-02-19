mod rawdecl;
mod rawexpr;
mod rawstmt;

pub(crate) use rawexpr::type_alias_expr;
pub(crate) use rawstmt::type_alias_stmt;

pub use self::{
  // rawdecl::*,
  rawexpr::{
    RawArraySubscript, RawBinary, RawCStyleCast, RawCall, RawCompoundLiteral,
    RawConstant, RawMemberAccess, RawParen, RawSizeOf, RawSizeOfKind,
    RawTernary, RawUnary, RawUnaryKind,
  },
  rawstmt::{
    RawBreak, RawCase, RawCompound, RawContinue, RawDefault, RawDoWhile,
    RawFor, RawGoto, RawIf, RawLabel, RawReturn, RawSwitch, RawWhile,
  },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Placeholder;

impl From<Placeholder> for () {
  #[inline(always)]
  fn from(_: Placeholder) -> Self {}
}
impl From<()> for Placeholder {
  #[inline(always)]
  fn from(_: ()) -> Self {
    Self
  }
}
impl ::std::fmt::Display for Placeholder {
  #[inline(always)]
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "")
  }
}
