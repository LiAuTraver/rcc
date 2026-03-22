#![feature(adt_const_params)]

mod arena;
mod constant;
mod diagnosis;
mod keyword;
mod operator;
mod source_info;
mod storage;
mod token;

pub use fwd::*;
mod fwd {
  pub use ::bumpalo::{
    self, Bump,
    collections::{
      CollectIn, CollectionAllocErr, FromIteratorIn, String as ArenaString,
      Vec as ArenaVec,
    },
  };
}

pub use self::{
  arena::Arena,
  constant::{Constant, ConstantRef, ConstantRefMut},
  diagnosis::{
    Data as DiagData, Diag, Diagnosis, Meta as DiagMeta, NoOp as NoOpDiag,
    Operational as OpDiag, Severity,
  },
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
