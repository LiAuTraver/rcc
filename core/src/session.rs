use ::bumpalo::Bump;
use ::std::rc::Rc;

use crate::{common::SourceManager, diagnosis::Operational, types::Type};

#[derive(Debug)]
pub struct Session {
  pub diagnosis: Operational,
  pub manager: Rc<SourceManager>,
}

impl Session {
  pub fn new(manager: Rc<SourceManager>) -> Self {
    Self {
      diagnosis: Operational::default(),
      manager,
    }
  }

  pub fn no_manager() -> Self {
    Self {
      diagnosis: Operational::default(),
      manager: Rc::new(SourceManager::default()),
    }
  }
}

#[derive(Default)]
pub struct Context {
  pub arena: Bump,
}
impl Context {
  pub fn new() -> Self {
    Default::default()
  }
}
