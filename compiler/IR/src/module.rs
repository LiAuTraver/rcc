use ::rcc_shared::{FileId, Triple};

use crate::{DataLayout, ValueID};

#[derive(Debug)]
pub struct Module<'d> {
  pub file_index: FileId,
  pub triple: Triple,
  pub data_layout: &'d DataLayout,
  /// global function and variable entry. Shall be either [`Function`] or [`Variable`], or [`Constant`].
  pub globals: Vec<ValueID>,
}

impl<'d> Module<'d> {
  pub fn new_empty(
    file_index: FileId,
    triple: Triple,
    data_layout: &'d DataLayout,
  ) -> Self {
    Self {
      file_index,
      triple,
      data_layout,
      globals: Default::default(),
    }
  }
}
