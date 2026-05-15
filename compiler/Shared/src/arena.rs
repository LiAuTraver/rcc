use ::bumpalo::Bump;
use ::std::{
  cell::RefCell,
  mem::{MaybeUninit, needs_drop},
  ptr::drop_in_place,
};

use super::Bumper;

type DropFn = unsafe fn(*mut u8);
#[derive(Debug, Default)]
pub struct Arena {
  bump: Bump,
  registry: RefCell<Vec<(*mut u8, DropFn)>>,
  #[cfg(debug_assertions)]
  counter: RefCell<usize>,
}

impl Bumper for Arena {
  #[inline(always)]
  fn alloc_uninit<T>(&self) -> *mut T {
    self.alloc(MaybeUninit::uninit()).as_mut_ptr()
  }

  fn alloc<T: ::std::fmt::Debug>(&self, val: T) -> &mut T {
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

  #[inline(always)]
  fn alloc_slice<T: Copy>(&self, values: &[T]) -> &mut [T] {
    self.bump.alloc_slice_copy(values)
  }

  #[inline(always)]
  fn alloc_str(&self, src: &str) -> &mut str {
    self.bump.alloc_str(src)
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
impl Drop for Arena {
  fn drop(&mut self) {
    unsafe {
      self
        .registry
        .borrow()
        .iter()
        .rev()
        .for_each(|(ptr, drop_fn)| drop_fn(*ptr));
    }
  }
}

impl Arena {
  #[inline(always)]
  pub(crate) fn raw_bump(&self) -> &Bump {
    &self.bump
  }

  #[inline(always)]
  pub fn new() -> Self {
    Default::default()
  }
}
