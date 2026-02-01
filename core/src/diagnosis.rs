mod data;
mod error;
mod warning;

use ::std::cell::{Ref, RefCell};

pub use self::data::{Data as DiagData, Diag, Meta as DiagMeta, Severity};
use crate::common::SourceSpan;

pub trait Diagnosis {
  #[must_use]
  fn has_errors(&self) -> bool;
  #[must_use]
  fn has_warnings(&self) -> bool;
  #[must_use]
  fn errors(&self) -> Ref<'_, Vec<Diag>>;
  #[must_use]
  fn warnings(&self) -> Ref<'_, Vec<Diag>>;
  fn add_error(&self, error: DiagData, span: SourceSpan);
  fn add_warning(&self, warning: DiagData, span: SourceSpan);
  fn add_diag(&self, diag: Diag) {
    match diag.metadata.severity {
      Severity::Error => self.add_error(diag.metadata.data, diag.span),
      Severity::Warning => self.add_warning(diag.metadata.data, diag.span),
      Severity::Info => {}, // ignore info for now
    }
  }
}

pub fn operational() -> OperationalDiag {
  OperationalDiag::default()
}

#[derive(Default, Debug)]

pub struct OperationalDiag {
  warnings: RefCell<Vec<Diag>>,
  errors: RefCell<Vec<Diag>>,
}

impl Diagnosis for OperationalDiag {
  #[inline]
  fn has_errors(&self) -> bool {
    !self.errors.borrow().is_empty()
  }

  #[inline]
  fn has_warnings(&self) -> bool {
    !self.warnings.borrow().is_empty()
  }

  #[inline]
  fn errors(&self) -> Ref<'_, Vec<Diag>> {
    self.errors.borrow()
  }

  #[inline]
  fn warnings(&self) -> Ref<'_, Vec<Diag>> {
    self.warnings.borrow()
  }

  #[inline]
  fn add_error(&self, error: DiagData, span: SourceSpan) {
    self
      .errors
      .borrow_mut()
      .push(Diag::new(span, Severity::Error, error));
  }

  #[inline]
  fn add_warning(&self, data: DiagData, span: SourceSpan) {
    self
      .warnings
      .borrow_mut()
      .push(Diag::new(span, Severity::Warning, data));
  }
}
#[derive(Default, Debug)]
pub struct Session {
  pub diagnosis: OperationalDiag,
}
