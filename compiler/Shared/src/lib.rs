#![feature(adt_const_params)]
#![feature(const_trait_impl)]
#![feature(const_cmp)]
mod arena;
mod bumper;
mod diagnosis;
mod keyword;
mod langopts;
mod number;
mod operator;
mod redeclarable;
mod source_info;
mod storage;
mod token;
mod triple;

#[doc(inline)]
pub use ::bumpalo::collections::{String as ArenaString, Vec as ArenaVec};

pub use self::{
  arena::Arena,
  bumper::{Bumper, CollectIn},
  diagnosis::{
    Data as DiagData, Diag, Diagnosis, Meta as DiagMeta, NoOp as NoOpDiag,
    Operational as OpDiag, Severity,
  },
  keyword::Keyword,
  langopts::LangOpts,
  number::Number,
  operator::{Category as OperatorCategory, Operator},
  redeclarable::{IntrusiveRedeclarableLink, Redeclarable},
  source_info::{
    Coordinate, Display as SourceDisplay, File as SourceFile,
    Id as SourceFileId, Id as FileId, Index as SourceSpanIndex,
    Index as SpanIndex, Manager as SourceManager, Span as SourceSpan,
    SpanDisplay,
  },
  storage::Storage,
  token::{Literal, Token},
  triple::{
    Architecture, CallingConvention, DataModel, Endianess, Environment,
    ObjectFormat, OperatingSystem, Triple, Vendor,
  },
};
