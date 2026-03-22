use ::rcc_shared::{Diagnosis, SourceManager};

use super::Context;

#[derive(Debug)]
pub struct Session<'c, D: Diagnosis<'c>> {
  diagnosis: &'c D,
  manager: &'c SourceManager,
  ast_context: &'c Context<'c>,
}

pub type SessionRef<'c, D> = &'c Session<'c, D>;

impl<'c, D: Diagnosis<'c>> Session<'c, D> {
  pub fn new(
    diagnosis: &'c D,
    manager: &'c SourceManager,
    ast_context: &'c Context<'c>,
  ) -> Self {
    Self {
      diagnosis,
      manager,
      ast_context,
    }
  }
}
impl<'c, D: Diagnosis<'c>> Session<'c, D> {
  pub fn ast(&self) -> &'c Context<'c> {
    self.ast_context
  }

  pub fn diag(&self) -> &'c D {
    self.diagnosis
  }

  pub fn src(&self) -> &'c SourceManager {
    self.manager
  }
}
