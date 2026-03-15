use crate::{common::SourceManager, diagnosis::Operational, ir, types};

#[derive(Debug)]
pub struct Session<'source, 'context>
where
  'source: 'context,
{
  pub diagnosis: Operational<'context>,
  pub manager: &'source SourceManager,
  pub ast_context: &'context types::Context<'context>,
  pub ir_context: &'context ir::Context<'context>,
}

impl<'source, 'context> Session<'source, 'context> {
  pub fn new(
    manager: &'source SourceManager,
    ast_context: &'context types::Context<'context>,
    ir_context: &'context ir::Context<'context>,
  ) -> Self {
    Self {
      diagnosis: Operational::default(),
      manager,
      ast_context,
      ir_context,
    }
  }
}
