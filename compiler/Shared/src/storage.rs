use super::Keyword;

::bitflags::bitflags! {
  /// storage-class-specifier
  #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
  pub struct StorageSpecifier: u8 {
    const Auto = 0x01;
    /// 6.7.2p17: \[...] has its value permanently fixed at translation-time;
    /// if not yet present, a `const`-qualification is implicitly added to the object’s type.
    const Constexpr = 0x02;
    /// 6.7.2p13: The declaration of an identifier for a function that has block scope
    /// shall have no explicit storage-class specifier other than extern.
    const Extern = 0x04;
    /// 6.7.2p12: The implementation can treat any `register` declaration simply as an `auto` declaration.
    ///
    /// However, \[...], the address of any part of an object declared with
    /// storage-class specifier `register` cannot be computed, \[...].
    ///
    /// Thus, the only operator that can be applied to an array declared with storage-class
    /// specifier `register` is `sizeof` and the `typeof` operators.
    ///
    /// In here, we just keep is as sematic check, then convert variable declared with `register` into automatic storage.
    const Register = 0x08;
    /// - At file scope, the static specifier indicates that a function or variable
    ///   has internal linkage.
    /// - At block scope(i.e., for variables), the static specifier controls storage duration, not linkage.
    const Static = 0x10;
    /// the variable is allocated when the thread is created
    const ThreadLocal = 0x20;
    /// according to standard, `typedef` is categorized as a storage-class specifier for *syntactic convenience only*.
    const Typedef = 0x40;

    /// 6.7.2p2: `thread_local` may appear with `static` or `extern`.
    const StaticThreadLocal = Self::Static.bits() | Self::ThreadLocal.bits();
    const ExternThreadLocal = Self::Extern.bits() | Self::ThreadLocal.bits();
    /// 6.7.2p2: `constexpr` may appear with `auto`, `register`, or `static`.
    const StaticConstexpr = Self::Static.bits() | Self::Constexpr.bits();
    const AutoConstexpr = Self::Auto.bits() | Self::Constexpr.bits();
    const RegisterConstexpr = Self::Register.bits() | Self::Constexpr.bits();
  }
}
impl StorageSpecifier {
  #[allow(non_upper_case_globals)]
  pub const Empty: Self = Self::empty();

  #[inline(always)]
  pub const fn reset(&mut self) {
    *self = Self::empty();
  }
}
impl TryFrom<Keyword> for StorageSpecifier {
  type Error = ();

  #[inline]
  fn try_from(kw: Keyword) -> Result<Self, Self::Error> {
    use Keyword::*;
    match kw {
      Auto => Ok(Self::Auto),
      Register => Ok(Self::Register),
      Extern => Ok(Self::Extern),
      Static => Ok(Self::Static),
      Typedef => Ok(Self::Typedef),
      ThreadLocal => Ok(Self::ThreadLocal),
      Constexpr => Ok(Self::Constexpr),
      _ => Err(()),
    }
  }
}

impl PartialEq<StorageSpecifier> for &StorageSpecifier {
  #[inline(always)]
  fn eq(&self, other: &StorageSpecifier) -> bool {
    **self == *other
  }
}

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
pub enum Linkage {
  /// [`StorageSpecifier::Auto`], No linkage.
  None,
  /// Tentative.
  Common,
  // /// Anonymous data like string literals and const arrays. Also [`Storage::Static`]. unused, may use in future
  // Private,
  /// Not visible, [`Storage::Static`].
  Internal,
  /// Visible to other translation units, [`Storage::Extern`].
  External,
}

impl Linkage {
  pub fn try_merge(prev: Self, incoming: Self) -> Result<Self, &'static str> {
    use Linkage::*;

    match (prev, incoming) {
      (None, None)
      | (Common, Common)
      | (Internal, Internal)
      | (External, External) => Ok(prev),

      (None, Common | Internal | External)
      | (Common | Internal | External, None) => panic!(
        "internal error; variable which has auto storage shall not call this \
         function"
      ),
      // unspecified
      (Internal, Common) =>
        Err("static declaration followed by non-static(tentative) declaration"),
      (Common, Internal) =>
        Err("non-static(tentative) declaration followed by static declaration"),
      // error
      (External, Internal) =>
        Err("non-static(extern) declaration followed by static declaration"),
      // extern is compatible with any other storage class; tentative are defacto placeholder.
      (Internal, External) => Ok(Internal),
      (External, Common) => Ok(Common),
      (Common, External) => Ok(External),
    }
  }
}
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
pub enum Storage {
  Static,
  Register,
  Automatic,
  ThreadLocal,
}

mod fmt {
  use ::std::fmt;

  use super::*;
  impl fmt::Display for StorageSpecifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      let mut first = true;
      for (name, _) in self.iter_names() {
        if !first {
          write!(f, " ")?;
        }
        write!(f, "{}", name.to_lowercase())?;
        first = false;
      }

      if first {
        write!(f, "<empty>")?;
      }
      Ok(())
    }
  }
}
