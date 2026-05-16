#![feature(adt_const_params)]
#![feature(const_trait_impl)]
#![feature(const_cmp)]
mod diagnosis;
mod keyword;
mod langopts;
mod number;
mod operator;
mod source_info;
mod storage;
mod token;
mod triple;

pub use self::{
  diagnosis::{
    Data as DiagData, Diag, Diagnosis, Meta as DiagMeta, NoOp as NoOpDiag,
    Operational as OpDiag, Severity,
  },
  keyword::Keyword,
  langopts::{C, Kind as Lang, Options as LangOpts, SysY},
  number::Number,
  operator::{Category as OperatorCategory, Operator},
  source_info::{
    Coordinate, Display as SourceDisplay, File as SourceFile,
    Id as SourceFileId, Id as FileId, Index as SourceSpanIndex,
    Index as SpanIndex, Manager as SourceManager, Span as SourceSpan,
    SpanDisplay,
  },
  storage::{Linkage, Storage, StorageSpecifier},
  token::{Literal, Token},
  triple::{
    Architecture, CallingConvention, DataModel, Endianess, Environment,
    ObjectFormat, OperatingSystem, Triple, Vendor,
  },
};
