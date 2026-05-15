use super::{DiagData, Keyword, Literal};

/// storage-class-specifier
#[derive(
  Debug,
  Clone,
  Copy,
  PartialEq,
  Eq,
  Hash,
  ::strum_macros::Display,
  ::strum_macros::EnumString,
  ::strum_macros::AsRefStr,
  ::strum_macros::IntoStaticStr,
)]
#[must_use]
#[strum(serialize_all = "snake_case")]
pub enum StorageSpecifier {
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
  Constexpr,
}

use StorageSpecifier::*;

impl TryFrom<Keyword> for StorageSpecifier {
  type Error = ();

  fn try_from(kw: Keyword) -> Result<Self, Self::Error> {
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

impl TryFrom<&Literal<'_>> for StorageSpecifier {
  type Error = ();

  fn try_from(literal: &Literal) -> Result<Self, Self::Error> {
    match literal {
      Literal::Keyword(kw) => Self::try_from(*kw),
      _ => Err(()),
    }
  }
}

impl StorageSpecifier {
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

  #[inline(always)]
  pub fn is_static(self) -> bool {
    matches!(self, Static)
  }

  #[inline(always)]
  pub fn is_extern(self) -> bool {
    matches!(self, Extern)
  }

  #[inline(always)]
  pub fn is_thread_local(self) -> bool {
    matches!(self, ThreadLocal)
  }

  #[inline(always)]
  pub fn is_constexpr(self) -> bool {
    matches!(self, Constexpr)
  }

  #[inline(always)]
  pub fn is_typedef(self) -> bool {
    matches!(self, Typedef)
  }

  #[inline(always)]
  pub fn is_automatic(self) -> bool {
    matches!(self, Automatic)
  }

  #[inline(always)]
  pub fn is_register(self) -> bool {
    matches!(self, Register)
  }
}

impl PartialEq<StorageSpecifier> for &StorageSpecifier {
  #[inline(always)]
  fn eq(&self, other: &StorageSpecifier) -> bool {
    **self == *other
  }
}

#[derive(Debug, Clone, Copy)]
pub enum Linkage {
  /// Visible to other translation units, [`Storage::Extern`].
  External,
  /// Not visible, [`Storage::Static`].
  Internal,
  /// Anonymous data like string literals and const arrays. Also [`Storage::Static`].
  Private,
  // /// Tentative.
  // Common,
}
#[derive(Debug, Clone, Copy)]
pub enum Storage {
  Static,
  Register,
  Automatic,
  ThreadLocal,
}
// impl From<Storage> for StorageDuration {
//   fn from(storage: Storage) -> Self {
//     use Storage::*;
//     match storage {
//       Automatic => Self::Automatic,
//       Register => Self::Register,
//       Extern => todo!(),
//       Static => Self::Static,
//       Typedef => panic!(""),
//       ThreadLocal => Self::ThreadLocal,
//       Constexpr => Self::Static,
//     }
//   }
// }
impl From<StorageSpecifier> for Linkage {
  fn from(storage: StorageSpecifier) -> Self {
    use Linkage::*;
    use StorageSpecifier::*;
    match storage {
      Extern => External,
      Static => Internal,
      _ => panic!("not a truly storage class."),
    }
  }
}
