use crate::{common::SourceManager, diagnosis::Operational};

#[derive(Debug)]
pub struct Session<'context, 'source>
where
  'source: 'context,
{
  pub diagnosis: Operational<'context>,
  pub manager: &'source SourceManager,
}

impl<'context, 'source> Session<'context, 'source> {
  pub fn new(manager: &'source SourceManager) -> Self {
    Self {
      diagnosis: Operational::default(),
      manager,
    }
  }
}
