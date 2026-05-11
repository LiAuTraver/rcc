/// *Best-effort* calculation result.
#[derive(Debug)]
pub enum Evaluation<T> {
  /// Fully evaluated
  Success(T),
  /// Partially evaluated
  Failure(T),
}
use Evaluation::{Failure, Success};

impl<T> Evaluation<T> {
  #[inline]
  pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Evaluation<U> {
    match self {
      Success(v) => Success(f(v)),
      Failure(v) => Failure(f(v)),
    }
  }

  #[inline]
  pub fn inspect_error<F>(self, f: F) -> Self
  where
    F: FnOnce(&T),
  {
    if let Failure(v) = &self {
      f(v)
    }
    self
  }

  /// This function **won't** panic, and always returns the inner value regardless of success or failure.
  #[inline]
  pub fn take(self) -> T {
    match self {
      Failure(v) | Success(v) => v,
    }
  }

  #[inline]
  pub fn transform<U>(self, f: impl FnOnce(T) -> U) -> U {
    match self {
      Success(v) | Failure(v) => f(v),
    }
  }
}
