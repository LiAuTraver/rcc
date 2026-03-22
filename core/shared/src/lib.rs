#![feature(adt_const_params)]

mod constant;
mod diagnosis;
mod keyword;
mod operator;
mod source_info;
mod storage;
mod token;

pub use self::{
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
