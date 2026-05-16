use ::std::{cell::RefCell, mem::needs_drop, ptr::drop_in_place};

type DropFn = unsafe fn(*mut u8);

/// runs [`drop_in_place`] when the registry is dropped. This is used to drop values allocated in the bump.
///
/// ## Warning
/// **so this struct shall be dropped before the bump allocator is dropped.**
/// Otherwise, `use-after-free` errors may occur.
///
/// make the registry a field before the bump allocator to ensure that.
#[derive(Debug, Default)]
pub(crate) struct Registry<const CHECK_THRESHOLD: bool = true> {
  inner: RefCell<Vec<(*mut u8, DropFn)>>,
  #[cfg(debug_assertions)]
  counter: RefCell<usize>,
}

impl<const C: bool> Registry<C> {
  #[cfg(debug_assertions)]
  #[inline(always)]
  fn _check_threshold(&self) {
    if const { C } {
      static THRESHOLD: usize = 16;
      *self.counter.borrow_mut() += 1;
      if *self.counter.borrow() >= THRESHOLD {
        eprintln!(
          "Error: registered too much needs_drop elems into the bump; perhaps \
           you bumped the wrong type? {}",
          self.counter.borrow()
        );
      }
    }
  }

  #[cfg(not(debug_assertions))]
  #[inline(always)]
  fn _check_threshold(&self) {}

  pub(crate) fn register<T>(&self, ptr: *mut T) {
    if const { needs_drop::<T>() } {
      self._check_threshold();

      #[inline]
      unsafe fn drop_fn<T>(ptr: *mut u8) {
        unsafe { drop_in_place(ptr as *mut T) };
      }

      self
        .inner
        .borrow_mut()
        .push((&raw mut *ptr as *mut u8, drop_fn::<T>));
    }
  }
}

impl<const C: bool> Drop for Registry<C> {
  fn drop(&mut self) {
    unsafe {
      self
        .inner
        .borrow()
        .iter()
        .rev()
        .for_each(|(ptr, drop_fn)| drop_fn(*ptr));
    }
  }
}
