/// delibreately write wrappers rather than using [`std::ops::Deref`] to get the [`bumpalo::Bump`].
#[allow(clippy::mut_from_ref, reason = "allocator is meant to do this.")]
pub trait BumpAllocator {
  /// Allocates uninitialized memory for a value of type `T` and returns a mutable reference to it.
  #[must_use]
  fn alloc_uninit<T>(&self) -> *mut T;

  /// If the returned reference is casted to a pointer, **beaware of the provenance**.
  #[must_use]
  fn alloc<T>(&self, val: T) -> &mut T;

  #[must_use]
  fn alloc_slice<T>(&self, values: &[T]) -> &mut [T]
  where
    T: Copy;

  #[must_use]
  fn alloc_slice_fill_iter<T, I>(&self, iter: I) -> &mut [T]
  where
    I: IntoIterator<Item = T>,
    I::IntoIter: ExactSizeIterator;
}
