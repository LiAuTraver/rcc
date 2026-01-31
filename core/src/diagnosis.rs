mod error;
mod warning;
use ::std::cell::{Ref, RefCell};

pub use self::{
  error::{Data as ErrorData, Error, ErrorDisplay},
  warning::{Data as WarningData, Warning, WarningDisplay},
};

pub trait Diagnosis {
  #[must_use]
  fn has_errors(&self) -> bool;
  #[must_use]
  fn has_warnings(&self) -> bool;
  #[must_use]
  fn errors(&self) -> Ref<'_, Vec<Error>>;
  #[must_use]
  fn warnings(&self) -> Ref<'_, Vec<Warning>>;
  fn add_error(&self, error: Error);
  fn add_warning(&self, warning: Warning);
}

pub fn operational() -> OperationalDiag {
  OperationalDiag::default()
}

#[derive(Default, Debug)]

pub struct OperationalDiag {
  errors: RefCell<Vec<Error>>,
  warnings: RefCell<Vec<Warning>>,
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
  fn errors(&self) -> Ref<'_, Vec<Error>> {
    self.errors.borrow()
  }

  #[inline]
  fn warnings(&self) -> Ref<'_, Vec<Warning>> {
    self.warnings.borrow()
  }

  #[inline]
  fn add_error(&self, error: Error) {
    self.errors.borrow_mut().push(error);
  }

  #[inline]
  fn add_warning(&self, warning: Warning) {
    self.warnings.borrow_mut().push(warning);
  }
}
#[derive(Default, Debug)]
pub struct Session {
  pub diagnosis: OperationalDiag,
}
