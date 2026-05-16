use ::bumpalo::Bump;
use ::std::marker::PhantomData;

use crate::Registry;

#[derive(Debug)]
pub struct DedicatedBumper<T> {
  registry: Registry<false>,
  bump: Bump,
  _serious_dedication: PhantomData<T>,
}

impl<T> Default for DedicatedBumper<T> {
  #[inline]
  fn default() -> Self {
    Self {
      bump: Default::default(),
      registry: Default::default(),
      _serious_dedication: Default::default(),
    }
  }
}
#[allow(clippy::mut_from_ref, reason = "allocator is meant to do this.")]
impl<T> DedicatedBumper<T> {
  #[inline(always)]
  pub fn new() -> Self {
    Self::default()
  }

  pub fn alloc(&self, val: T) -> &mut T {
    let ptr = self.bump.alloc(val) as *mut T;
    self.registry.register(ptr);
    unsafe { &mut *ptr }
  }
}
