use ::rcc_shared::{Diagnosis, SourceManager, Triple};
use ::std::ops::Deref;

use super::Context;

#[derive(Debug)]
pub struct Session<'c, D: Diagnosis<'c>> {
  diagnosis: &'c D,
  manager: &'c SourceManager,
  ast_context: &'c Context<'c>,
  triple: Triple,
}

pub type SessionRef<'c, D> = &'c Session<'c, D>;

impl<'c, D: Diagnosis<'c>> Session<'c, D> {
  pub fn new(
    diagnosis: &'c D,
    manager: &'c SourceManager,
    ast_context: &'c Context<'c>,
    triple: Triple,
  ) -> Self {
    Self {
      diagnosis,
      manager,
      ast_context,
      triple,
    }
  }
}
impl<'c, D: Diagnosis<'c>> Session<'c, D> {
  #[inline]
  pub fn ast(&self) -> &'c Context<'c> {
    self.ast_context
  }

  #[inline]
  pub fn diag(&self) -> &'c D {
    self.diagnosis
  }

  #[inline]
  pub fn src(&self) -> &'c SourceManager {
    self.manager
  }

  #[inline]
  pub fn triple(&self) -> Triple {
    self.triple
  }
}

impl<'c, D: Diagnosis<'c>> Deref for Session<'c, D> {
  type Target = Context<'c>;

  #[inline]
  fn deref(&self) -> &'c Self::Target {
    self.ast_context
  }
}
