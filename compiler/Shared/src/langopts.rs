#[derive(Debug)]
pub struct Options {
  lang: Kind,
}

impl Options {
  #[inline(always)]
  pub fn new(lang: impl Into<Kind>) -> Self {
    Self { lang: lang.into() }
  }
}

#[derive(Debug, Clone, Copy)]
pub enum Kind {
  C(C),
  SysY(SysY),
}
#[derive(
  Debug,
  Clone,
  Copy,
  PartialEq,
  Eq,
  PartialOrd,
  Ord,
  ::strum_macros::Display,
  ::strum_macros::EnumString,
  ::strum_macros::AsRefStr,
  ::strum_macros::IntoStaticStr,
)]
pub enum C {
  C99,
  C11,
  C17,
  C23,
}

#[derive(
  Debug,
  Clone,
  Copy,
  PartialEq,
  Eq,
  PartialOrd,
  Ord,
  ::strum_macros::Display,
  ::strum_macros::EnumString,
  ::strum_macros::AsRefStr,
  ::strum_macros::IntoStaticStr,
)]
pub enum SysY {
  SysY2026,
}

mod cvt {
  use ::std::ops::Deref;

  use super::*;

  impl Kind {
    #[inline(always)]
    pub fn is_c_and(&self, f: impl FnOnce(C) -> bool) -> bool {
      match self {
        Kind::C(c) => f(*c),
        _ => false,
      }
    }

    #[inline(always)]
    pub fn is_sysy_and(&self, f: impl FnOnce(SysY) -> bool) -> bool {
      match self {
        Kind::SysY(sysy) => f(*sysy),
        _ => false,
      }
    }
  }
  impl From<C> for Kind {
    #[inline(always)]
    fn from(c: C) -> Self {
      Self::C(c)
    }
  }
  impl From<SysY> for Kind {
    #[inline(always)]
    fn from(sysy: SysY) -> Self {
      Self::SysY(sysy)
    }
  }

  impl Deref for Options {
    type Target = Kind;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
      &self.lang
    }
  }
}
