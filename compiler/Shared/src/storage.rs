use super::{DiagData, Keyword, Literal};

/// storage-class-specifier
#[derive(Debug, ::strum_macros::Display, PartialEq, Eq, Clone, Copy, Hash)]
#[strum(serialize_all = "snake_case")]
pub enum Storage {
  /// variables that declared in block scope without any storage-class specifier
  /// are considered to have automatic storage duration.
  #[strum(serialize = "auto")]
  Automatic,
  /// 6.7.2p12: The implementation can treat any `register` declaration simply as an `auto` declaration.
  ///
  /// However, \[...], the address of any part of an object declared with
  /// storage-class specifier `register` cannot be computed, \[...].
  ///
  /// Thus, the only operator that can be applied to an array declared with storage-class
  /// specifier `register` is `sizeof` and the `typeof` operators.
  ///
  /// In here, we just keep is as sematic check, then convert variable declared with `register` into automatic storage.
  Register,
  /// - Function declarations with no storage-class specifier are always handled
  ///   as though they include an extern specifier
  /// - if variable declarations appear at file scope, they have external linkage
  /// - use extern to declare an identifier that’s already visible.
  Extern,
  /// - At file scope, the static specifier indicates that a function or variable
  ///   has internal linkage.
  /// - At block scope(i.e., for variables), the static specifier controls storage duration, not linkage.
  Static,
  /// according to standard, `typedef` is categorized as a storage-class specifier for *syntactic convenience only*.
  Typedef,
  /// the variable is allocated when the thread is created
  ///
  /// like typedef, this is classified as storage-class specifier for *syntactic convenience only* as well.
  /// it acts more like an attribute. if I were to implement it, pls add to attribute.
  ThreadLocal,
  /// C23.
  Constexpr, // ignore for now
}

use Storage::*;

impl TryFrom<&Keyword> for Storage {
  type Error = ();

  fn try_from(kw: &Keyword) -> Result<Self, Self::Error> {
    match kw {
      Keyword::Auto => Ok(Automatic),
      Keyword::Register => Ok(Register),
      Keyword::Extern => Ok(Extern),
      Keyword::Static => Ok(Static),
      Keyword::Typedef => Ok(Typedef),
      Keyword::ThreadLocal => Ok(ThreadLocal),
      Keyword::Constexpr => Ok(Constexpr),
      _ => Err(()),
    }
  }
}

impl<'c> TryFrom<&Literal<'c>> for Storage {
  type Error = ();

  fn try_from(literal: &Literal) -> Result<Self, Self::Error> {
    match literal {
      Literal::Keyword(kw) => Self::try_from(kw),
      _ => Err(()),
    }
  }
}

impl Storage {
  pub fn try_merge<'c>(
    prev: Self,
    incoming: Self,
  ) -> Result<Self, DiagData<'c>> {
    use DiagData::*;
    match (prev, incoming) {
      (lhs, rhs) if lhs == rhs => Ok(lhs),
      // actually it's an error, but clang promoted, anyways
      (Extern, _) => Ok(Extern),
      // extern is compatible with any other storage class
      (other, Extern) => Ok(other),
      (Typedef | ThreadLocal, _) | (_, Typedef | ThreadLocal) =>
        Err(StorageSpecsUnmergeable(prev, incoming)),
      (Constexpr, _) | (_, Constexpr) => Err(UnsupportedFeature(
        "Constexpr unimplemented yet".to_string(),
      )),
      _ => Err(StorageSpecsUnmergeable(prev, incoming)),
    }
  }

  #[inline]
  pub fn is_static(&self) -> bool {
    matches!(self, Static)
  }

  #[inline]
  pub fn is_extern(&self) -> bool {
    matches!(self, Extern)
  }

  #[inline]
  pub fn is_thread_local(&self) -> bool {
    matches!(self, ThreadLocal)
  }

  #[inline]
  pub fn is_constexpr(&self) -> bool {
    matches!(self, Constexpr)
  }

  #[inline]
  pub fn is_typedef(&self) -> bool {
    matches!(self, Typedef)
  }

  #[inline]
  pub fn is_automatic(&self) -> bool {
    matches!(self, Automatic)
  }

  #[inline]
  pub fn is_register(&self) -> bool {
    matches!(self, Register)
  }
}

impl PartialEq<Storage> for &Storage {
  fn eq(&self, other: &Storage) -> bool {
    **self == *other
  }
}
