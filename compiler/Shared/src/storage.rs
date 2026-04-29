use super::{DiagData, DiagMeta, Keyword, Literal, Severity};

/// storage-class-specifier
#[derive(Debug, ::strum_macros::Display, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Storage {
  /// variables that declared in block scope without any storage-class specifier
  /// are considered to have automatic storage duration.
  #[strum(serialize = "auto")]
  Automatic,
  /// 6.7.2.12: The implementation can treat any `register` declaration simply as an `auto` declaration.
  /// However, \[...], the address of any part of an object declared with storage-class specifier `register` cannot be computed, \[...]
  ///
  /// Thus, the only operator that can be applied to an array declared with storage-class
  /// specifier register is `sizeof` and the `typeof` operators.
  ///
  /// In here, we just keep is as sematic check, then convert variable declared with `register` into `auto` storage.
  #[strum(serialize = "register")]
  Register,
  /// - Function declarations with no storage-class specifier are always handled
  ///   as though they include an extern specifier
  /// - if variable declarations appear at file scope, they have external linkage
  /// - use extern to declare an identifier that’s already visible.
  /// ```c
  /// static int a;
  /// extern int a; // this is valid and a has internal linkage
  /// extern int b;
  /// static int b = 0; // this is also valid... (internal linkage)
  /// ```
  #[strum(serialize = "extern")]
  Extern,
  /// - At file scope, the static specifier indicates that a function or variable
  ///   has internal linkage.
  /// - At block scope(i.e., for variables), the static specifier controls storage duration, not linkage.
  #[strum(serialize = "static")]
  Static,
  /// according to standard, `typedef` is categorized as a storage-class specifier for **syntactic convenience only**.
  #[strum(serialize = "typedef")]
  Typedef,
  /// the variable is allocated when the thread is created
  #[strum(serialize = "thread_local")]
  ThreadLocal, // I won't care about this now
  /// C23, `#define VAR value` is the same `constexpr TYPE VAR = value;` with fewer name collisions
  #[strum(serialize = "constexpr")]
  Constexpr, // ditto
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
    lhs: &Storage,
    rhs: &Storage,
  ) -> Result<Storage, DiagMeta<'c>> {
    use DiagData::*;
    use Severity::*;
    match (lhs, rhs) {
      (lhs, rhs) if lhs == rhs => Ok(*lhs),
      (Constexpr, _) | (_, Constexpr) => Err(
        UnsupportedFeature("Constexpr unimplemented yet".to_string()) + Error,
      ),
      (Typedef, _) | (_, Typedef) =>
        Err(StorageSpecsUnmergeable(*lhs, *rhs) + Error),
      (Extern, other) | (other, Extern) => Ok(*other), // extern is compatible with any other storage class
      _ => Err(StorageSpecsUnmergeable(*lhs, *rhs) + Error),
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
