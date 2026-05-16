use ::bumpalo::{Bump, collections as bc};
use ::std::mem::MaybeUninit;

use crate::{BumpAllocator, Registry};

#[derive(Debug, Default)]
pub struct Arena {
  registry: Registry,
  bump: Bump,
}

impl BumpAllocator for Arena {
  #[inline(always)]
  fn alloc_uninit<T>(&self) -> *mut T {
    self.alloc(MaybeUninit::uninit()).as_mut_ptr()
  }

  fn alloc<T>(&self, val: T) -> &mut T {
    #[inline(always)]
    fn _print_meta<T>() {
      use ::std::any::type_name;
      println!("Allocating memory for {}.", type_name::<T>());
    }

    // _print_meta::<T>();

    let ptr = self.bump.alloc(val) as *mut T;
    self.registry.register(ptr);
    unsafe { &mut *ptr }
  }

  #[inline(always)]
  fn alloc_slice<T: Copy>(&self, values: &[T]) -> &mut [T] {
    self.bump.alloc_slice_copy(values)
  }

  #[inline(always)]
  fn alloc_slice_fill_iter<T, I>(&self, iter: I) -> &mut [T]
  where
    I: IntoIterator<Item = T>,
    I::IntoIter: ExactSizeIterator,
  {
    self.bump.alloc_slice_fill_iter(iter)
  }
}

impl Arena {
  #[inline(always)]
  pub fn new() -> Self {
    Default::default()
  }
}

/// See the doc of [`CollectIn`](bc::CollectIn). This is a re-export.
pub trait CollectIn<'bump>: Iterator + Sized {
  /// See the doc of [`collect_in`](bc::CollectIn::collect_in). This is a re-export.
  #[inline(always)]
  fn collect_in<C: bc::FromIteratorIn<Self::Item, Alloc = &'bump Bump>>(
    self,
    alloc: &'bump Arena,
  ) -> C {
    C::from_iter_in(self, &alloc.bump)
  }
}
impl<T> CollectIn<'_> for T where T: bc::CollectIn {}
