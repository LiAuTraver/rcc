use crate::{common::SourceManager, diagnosis::Operational, types::Context};

#[derive(Debug)]
pub struct Session<'source, 'context, 'ir>
where
  'source: 'context,
  'source: 'ir,
  'context: 'ir,
{
  pub diagnosis: Operational<'context>,
  pub manager: &'source SourceManager,
  pub ast_context: &'context Context<'context>,
  pub ir_context: &'ir crate::ir::Context<'context, 'ir>,
}

impl<'source, 'context, 'ir> Session<'source, 'context, 'ir> {
  pub fn new(
    manager: &'source SourceManager,
    ast_context: &'context Context<'context>,
    ir_context: &'ir crate::ir::Context<'context, 'ir>,
  ) -> Self {
    Self {
      diagnosis: Operational::default(),
      manager,
      ast_context,
      ir_context,
    }
  }
}
