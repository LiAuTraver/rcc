use crate::{common::SourceManager, diagnosis::Operational, types::Context};

#[derive(Debug)]
pub struct Session<'context, 'source>
where
  'source: 'context,
{
  pub diagnosis: Operational<'context>,
  pub manager: &'source SourceManager,
  pub context: &'context Context<'context>,
}

impl<'context, 'source> Session<'context, 'source> {
  pub fn new(
    manager: &'source SourceManager,
    context: &'context Context<'context>,
  ) -> Self {
    Self {
      diagnosis: Operational::default(),
      manager,
      context,
    }
  }
}
