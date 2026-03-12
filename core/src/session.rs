use crate::{common::SourceManager, diagnosis::Operational, types::Context};

#[derive(Debug)]
pub struct Session<'context, 'source>
where
  'source: 'context,
{
  pub diagnosis: Operational<'context>,
  pub manager: &'source SourceManager,
  pub ast_context: &'context Context<'context>,
  pub ir_context: &'context crate::ir::Context<'context>,
}

impl<'context, 'source> Session<'context, 'source> {
  pub fn new(
    manager: &'source SourceManager,
    ast_context: &'context Context<'context>,
    ir_context: &'context crate::ir::Context<'context>,
  ) -> Self {
    Self {
      diagnosis: Operational::default(),
      manager,
      ast_context,
      ir_context,
    }
  }
}
