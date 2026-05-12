use ::std::{
  cell::RefCell,
  mem::{MaybeUninit, needs_drop},
  ptr::drop_in_place,
};

use crate::Bump;

type DropFn = unsafe fn(*mut u8);
#[derive(Debug, Default)]
pub struct Arena {
  bump: Bump,
  registry: RefCell<Vec<(*mut u8, DropFn)>>,
  #[cfg(debug_assertions)]
  counter: RefCell<usize>,
}

/// delibreately write wrappers rather than use [`std::ops::Deref`].
impl Arena {
  /// Allocates uninitialized memory for a value of type `T` and returns a mutable reference to it.
  ///
  /// # Safety
  /// If the returned reference is casted to a pointer, **be aware of the provenance**
  /// and ensure there only exists one (mut) pointer to the same memory.
  ///
  /// *always use `cargo miri test` to test code that uses this function to ensure safety!*
  ///
  /// Unsafe operations w.r.t. the pointer would easily violate
  /// [Stacked Borrows](https://github.com/rust-lang/unsafe-code-guidelines/blob/master/wip/stacked-borrows.md).
  ///
  /// Note: `cargo miri` would sometimes fail on Windows since File APIs are unsupported.
  #[inline]
  #[must_use]
  pub unsafe fn alloc_uninit<T>(&self) -> *mut T {
    self.alloc(MaybeUninit::<T>::uninit()).as_mut_ptr()
  }

  /// If the returned reference is casted to a pointer, **beaware of the provenance**.
  #[must_use]
  #[allow(clippy::mut_from_ref, reason = "allocator is meant to do this.")]
  pub fn alloc<T: ::std::fmt::Debug>(&self, val: T) -> &mut T {
    #[inline(always)]
    fn _print_meta<T: ::std::fmt::Debug>(val: &T) {
      use ::std::any::type_name;
      println!("Allocating memory for {}:  {:?}", type_name::<T>(), val);
    }

    // _print_meta(&val);

    let ptr = self.bump.alloc(val) as *mut T;

    if const { needs_drop::<T>() } {
      #[cfg(debug_assertions)]
      #[inline(always)]
      fn _check_threshold(arena: &Arena) {
        static THRESHOLD: usize = 16;
        *arena.counter.borrow_mut() += 1;
        if *arena.counter.borrow() >= THRESHOLD {
          eprintln!(
            "Error: registered too much needs_drop elems into the bump; \
             perhaps you bumped the wrong type? {}",
            arena.counter.borrow()
          );
        }
      }
      #[cfg(not(debug_assertions))]
      #[inline(always)]
      fn _check_threshold<T>(_: &T) {}

      _check_threshold(self);

      #[inline]
      unsafe fn drop_fn<T>(ptr: *mut u8) {
        unsafe { drop_in_place(ptr as *mut T) };
      }

      self
        .registry
        .borrow_mut()
        .push((&raw mut *ptr as *mut u8, drop_fn::<T>));
    }

    unsafe { &mut *ptr }
  }

  #[inline]
  pub fn alloc_slice_copy<T: Copy>(&self, values: &[T]) -> &mut [T] {
    self.bump.alloc_slice_copy(values)
  }

  #[inline(always)]
  pub fn alloc_str(&self, src: &str) -> &mut str {
    self.bump.alloc_str(src)
  }

  #[inline(always)]
  pub fn raw_bump(&self) -> &Bump {
    &self.bump
  }

  #[inline(always)]
  pub fn alloc_slice_fill_iter<T, I>(&self, iter: I) -> &mut [T]
  where
    I: IntoIterator<Item = T>,
    I::IntoIter: ExactSizeIterator,
  {
    self.bump.alloc_slice_fill_iter(iter)
  }
}
impl Drop for Arena {
  fn drop(&mut self) {
    self
      .registry
      .borrow()
      .iter()
      .rev()
      .for_each(|(ptr, drop_fn)| unsafe {
        drop_fn(*ptr);
      });
  }
}
