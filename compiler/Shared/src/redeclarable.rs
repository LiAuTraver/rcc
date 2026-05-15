use ::rcc_utils::{ensure_is_pod, static_assert, static_assert_eq};
use ::std::{
  cell::Cell,
  mem::{MaybeUninit, offset_of},
  ops::Deref,
  ptr::NonNull,
};

use crate::Bumper;

/// A Redeclarable link.
#[repr(C)]
#[derive(Debug)]
pub struct Link<T> {
  /// points to the earlist appeared node.
  ///
  /// use as a **unique identifier** of the redeclarable chain.
  canonical: NonNull<T>,
  /// stores either the previous node or the latest node.
  ///
  /// - if the current node is the canonical one, it points to the latest node of the chain, acts as a **infromation hub**;
  /// - otherwise, it points to the previous node.
  prev_or_latest: Cell<NonNull<T>>,
}
impl<T> Link<T> {
  #[must_use]
  pub fn new(canonical: NonNull<T>, prev_or_latest: Cell<NonNull<T>>) -> Self {
    Self {
      canonical,
      prev_or_latest,
    }
  }
}
pub type IntrusiveRedeclarableLink<T> = Link<T>;
impl<T> IntrusiveRedeclarableLink<T> {
  #[must_use]
  #[inline(always)]
  pub fn canonical(&self) -> NonNull<T> {
    self.canonical
  }

  #[must_use]
  #[inline(always)]
  pub fn prev_or_latest(&self) -> NonNull<T> {
    self.prev_or_latest.get()
  }

  #[inline(always)]
  pub fn set_prev_or_latest(&self, ptr: NonNull<T>) {
    self.prev_or_latest.set(ptr);
  }
}
#[repr(C)]
#[derive(Debug)]
pub struct Redeclarable<T> {
  link: Link<Self>,
  data: T,
}
impl<T> Deref for Redeclarable<T> {
  type Target = T;

  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    &self.data
  }
}
static_assert!(
  offset_of!(Redeclarable<usize>, link) == 0,
  "repr(C) layout failed!"
);
static_assert_eq!(
  size_of::<Redeclarable<usize>>(),
  size_of::<Option<Redeclarable<usize>>>(),
  "[[no_unique_address]] failed!"
);
ensure_is_pod!(Redeclarable<usize>);
impl<T> Redeclarable<T> {
  #[must_use]
  #[inline]
  fn _new(
    canonical: NonNull<Self>,
    prev_or_latest: Cell<NonNull<Self>>,
    data: T,
  ) -> Self {
    Self {
      link: Link::new(canonical, prev_or_latest),
      data,
    }
  }

  #[must_use]
  #[inline(always)]
  pub fn new<'a>(
    bump: &'a impl Bumper,
    prev: Option<&Self>,
    data: T,
  ) -> &'a Self {
    if let Some(prev) = prev {
      Self::new_node(bump, prev, data)
    } else {
      Self::new_canonical(bump, data)
    }
  }

  #[must_use]
  pub fn new_node<'a>(bump: &'a impl Bumper, prev: &Self, data: T) -> &'a Self {
    let canonical = prev.link.canonical;
    let prev = Cell::new(NonNull::from(prev));
    let this = bump.alloc(MaybeUninit::<Self>::uninit()).as_mut_ptr();
    unsafe {
      this.write(Redeclarable::_new(canonical, prev, data));
      (*this)
        .link
        .canonical
        .as_ref()
        .link
        .prev_or_latest
        .set(NonNull::new_unchecked(this));
    }
    unsafe { &*this }
  }

  #[must_use]
  pub fn new_canonical(bump: &impl Bumper, data: T) -> &Self {
    let this = bump.alloc(MaybeUninit::<Self>::uninit()).as_mut_ptr();
    unsafe {
      this.write(Redeclarable::_new(
        NonNull::new_unchecked(this),
        Cell::new(NonNull::new_unchecked(this)),
        data,
      ));
    }
    unsafe { &*this }
  }
}

impl<T> Redeclarable<T> {
  #[must_use]
  #[inline(always)]
  pub fn canonical(&self) -> &Self {
    unsafe { self.link.canonical.as_ref() }
  }

  #[must_use]
  #[inline(always)]
  pub fn latest(&self) -> &Self {
    unsafe {
      self
        .link
        .canonical
        .as_ref()
        .link
        .prev_or_latest
        .get()
        .as_ref()
    }
  }

  #[must_use]
  #[inline(always)]
  pub fn prev(&self) -> Option<&Self> {
    if ::std::ptr::eq::<Self>(self, self.link.canonical.as_ptr()) {
      None
    } else {
      unsafe { Some(self.link.prev_or_latest.get().as_ref()) }
    }
  }
}
impl<T> Redeclarable<T> {
  #[must_use]
  #[inline(always)]
  pub fn is_canonical(&self) -> bool {
    ::std::ptr::eq::<Self>(self, self.link.canonical.as_ptr())
  }

  #[must_use]
  #[inline(always)]
  pub fn is_latest(&self) -> bool {
    ::std::ptr::eq::<Self>(self, unsafe {
      self
        .link
        .canonical
        .as_ref()
        .link
        .prev_or_latest
        .get()
        .as_ptr()
    })
  }

  #[must_use]
  #[inline(always)]
  pub fn iter(&self) -> RedeclarableIter<'_, T> {
    RedeclarableIter {
      current: Some(self),
    }
  }
}

pub struct RedeclarableIter<'a, T> {
  current: Option<&'a Redeclarable<T>>,
}
impl<'a, T> Iterator for RedeclarableIter<'a, T> {
  type Item = &'a Redeclarable<T>;

  #[inline(always)]
  fn next(&mut self) -> Option<Self::Item> {
    let current = self.current?;
    self.current = current.prev();
    Some(current)
  }
}
ensure_is_pod!(RedeclarableIter<'_, usize>);

/// A macro to generate a redeclarable struct with an intrusive link.
/// ```rust
/// use ::rcc_shared::{IntrusiveRedeclarableLink, make_intrusive_redeclarable_node};
///
/// #[repr(C)] // this is important.
/// #[derive(Debug)]
/// pub struct IntrusiveTest<'c> {
///   link: IntrusiveRedeclarableLink<Self>,
///   other_data: u8,
///   other_data_with_lifetime: &'c str,
/// }
/// make_intrusive_redeclarable_node!(link => pub IntrusiveTest[other_data: u8, other_data_with_lifetime: &'c str]: 'c);
/// ```
///
/// See the API of [`Redeclarable`] for the generated methods.
#[macro_export]
macro_rules! make_intrusive_redeclarable_node {
  ($link_field:ident => $visibility:ident $ty:ident[$($extra_field:ident: $extra_ty:ty),*]: $lt:lifetime) => {
    ::rcc_utils::static_assert!(
      ::std::mem::offset_of!($ty, $link_field) == 0,
      "the link field must be the first field of the struct! or maybe you forgot to add #[repr(C)]?"
    );
    ::rcc_utils::static_assert_eq!(
      ::std::mem::size_of::<$ty>(),
      ::std::mem::size_of::<Option<$ty>>(),
      "[[no_unique_address]] failed!"
    );
    ::rcc_utils::ensure_is_pod!($ty);
    impl<$lt> $ty<$lt> {
      #[must_use]
      #[inline(always)]
      pub fn canonical(&self) -> &Self {
        unsafe { self.$link_field.canonical().as_ref() }
      }

      #[must_use]
      #[inline(always)]
      pub fn latest(&self) -> &Self {
        unsafe {
          self
            .$link_field
            .canonical()
            .as_ref()
            .$link_field
            .prev_or_latest()
            .as_ref()
        }
      }

      #[must_use]
      #[inline(always)]
      pub fn prev(&self) -> Option<&Self> {
        if ::std::ptr::eq::<Self>(self, self.$link_field.canonical().as_ptr()) {
          None
        } else {
          unsafe { Some(self.$link_field.prev_or_latest().as_ref()) }
        }
      }

      #[must_use]
      #[inline(always)]
      pub fn is_canonical(&self) -> bool {
        ::std::ptr::eq::<Self>(self, self.$link_field.canonical().as_ptr())
      }

      #[must_use]
      #[inline(always)]
      pub fn is_latest(&self) -> bool {
        ::std::ptr::eq::<Self>(self, unsafe {
          self
            .$link_field
            .canonical()
            .as_ref()
            .$link_field
            .prev_or_latest()
            .as_ptr()
        })
      }

      #[must_use]
      #[inline(always)]
      #[allow(clippy::too_many_arguments)]
      fn _new(canonical: ::std::ptr::NonNull<Self>, prev_or_latest: ::std::cell::Cell<::std::ptr::NonNull<Self>>, $($extra_field: $extra_ty),*) -> Self {
        Self {
          $link_field: $crate::IntrusiveRedeclarableLink::new(canonical, prev_or_latest),
          $($extra_field),*
        }
      }

      #[must_use]
      #[inline(always)]
      #[allow(clippy::too_many_arguments)]
      pub fn new<'a>(bump: &'a impl $crate::Bumper, prev: Option<&Self>, $($extra_field: $extra_ty),*) -> &'a Self {
        if let Some(prev) = prev {
          Self::new_node(bump, prev, $($extra_field),*)
        } else {
          Self::new_canonical(bump, $($extra_field),*)
        }
      }

      #[must_use]
      #[inline]
      #[allow(clippy::too_many_arguments)]
      pub fn new_node<'a>(bump: &'a impl $crate::Bumper, prev: &Self, $($extra_field: $extra_ty),*) -> &'a Self {
        let canonical = prev.$link_field.canonical();
        let prev = ::std::cell::Cell::new(::std::ptr::NonNull::from(prev));
        let this = bump.alloc(::std::mem::MaybeUninit::<Self>::uninit()).as_mut_ptr();
        unsafe {
          this.write(Self::_new(canonical, prev, $($extra_field),*));
          (*this)
            .$link_field
            .canonical()
            .as_ref()
            .$link_field
            .set_prev_or_latest(::std::ptr::NonNull::new_unchecked(this));
        }
        unsafe { &*this }
      }

      #[must_use]
      #[inline]
      #[allow(clippy::too_many_arguments)]
      pub fn new_canonical<'a>(bump: &'a impl $crate::Bumper, $($extra_field: $extra_ty),*) -> &'a Self {
        let this = bump.alloc(::std::mem::MaybeUninit::<Self>::uninit()).as_mut_ptr();
        unsafe {
          this.write(Self::_new(
            ::std::ptr::NonNull::new_unchecked(this),
            ::std::cell::Cell::new(::std::ptr::NonNull::new_unchecked(this)),
            $($extra_field),*
          ));
        }
        unsafe { &*this }
      }
    }

    ::rcc_utils::paste! {
      $visibility struct [<$ty Iter>]<'a, $lt> {
        current: Option<&'a $ty<$lt>>,
      }
      ::rcc_utils::ensure_is_pod!([<$ty Iter>]);
      impl<'a, $lt> Iterator for [<$ty Iter>]<'a, $lt> {
        type Item = &'a $ty<$lt>;

        #[inline(always)]
        fn next(&mut self) -> Option<Self::Item> {
          let current = self.current?;
          self.current = current.prev();
          Some(current)
        }
      }
      impl<$lt> $ty<$lt> {
        #[must_use]
        #[inline(always)]
        pub fn iter(&self) -> [<$ty Iter>]<'_, $lt> {
          [<$ty Iter>] {
            current: Some(self),
          }
        }
      }
    }
  };
}

#[cfg(test)]
mod tests {

  macro_rules! assert_node {
    ($lhs:expr, $rhs:expr) => {
      assert!(
        ::std::ptr::eq($lhs, $rhs),
        concat!(stringify!($lhs), ": {:p}, ", stringify!($rhs), ": {:p}"),
        $lhs,
        $rhs
      );
    };
  }

  use super::*;
  use crate::Arena;

  #[test]
  fn redeclarable() {
    let datas = [0, 1, 2];
    let bump = Arena::new();

    let head = Redeclarable::new_canonical(&bump, datas[0]);
    assert_eq!(head.data, datas[0]);

    assert_node!(head.canonical(), head);
    assert_node!(head.latest(), head);
    assert!(head.prev().is_none());
    assert!(head.is_canonical());
    assert!(head.is_latest());
    unsafe {
      assert_node!(head.link.prev_or_latest.get().as_ref(), head);
      assert_node!(head.link.canonical.as_ref(), head);
    }

    let second = Redeclarable::new_node(&bump, head, datas[1]);
    assert_eq!(second.data, datas[1]);

    assert_node!(second.canonical(), head);
    assert_node!(second.latest(), second);
    assert_node!(second.prev().unwrap(), head);
    assert!(!second.is_canonical());
    assert!(second.is_latest());
    unsafe {
      assert_node!(second.link.prev_or_latest.get().as_ref(), head);
      assert_node!(second.link.canonical.as_ref(), head);
    }

    assert_node!(head.canonical(), head);
    assert_node!(head.latest(), second);
    assert!(head.prev().is_none());
    assert!(head.is_canonical());
    assert!(!head.is_latest());
    unsafe {
      assert_node!(head.link.prev_or_latest.get().as_ref(), second);
      assert_node!(head.link.canonical.as_ref(), head);
    }

    let third = Redeclarable::new_node(&bump, second, datas[2]);
    assert_eq!(third.data, datas[2]);

    assert_node!(third.canonical(), head);
    assert_node!(third.latest(), third);
    assert_node!(third.prev().unwrap(), second);
    assert!(!third.is_canonical());
    assert!(third.is_latest());
    unsafe {
      assert_node!(third.link.prev_or_latest.get().as_ref(), second);
      assert_node!(third.link.canonical.as_ref(), head);
    }

    assert_node!(second.canonical(), head);
    assert_node!(second.latest(), third);
    assert_node!(second.prev().unwrap(), head);
    assert!(!second.is_canonical());
    assert!(!second.is_latest());
    unsafe {
      assert_node!(second.link.prev_or_latest.get().as_ref(), head);
      assert_node!(second.link.canonical.as_ref(), head);
    }

    assert_node!(head.canonical(), head);
    assert_node!(head.latest(), third);
    assert!(head.prev().is_none());
    assert!(head.is_canonical());
    assert!(!head.is_latest());
    unsafe {
      assert_node!(head.link.prev_or_latest.get().as_ref(), third);
      assert_node!(head.link.canonical.as_ref(), head);
    }

    let mut iter = third.iter();
    assert_eq!(iter.next().unwrap().data, datas[2]);
    assert_eq!(iter.next().unwrap().data, datas[1]);
    assert_eq!(iter.next().unwrap().data, datas[0]);
    assert!(iter.next().is_none());
    assert!(iter.next().is_none());
  }
  pub struct IntrusiveTest<'c> {
    link: IntrusiveRedeclarableLink<Self>,
    field1: u8,
    field2: &'c str,
  }
  make_intrusive_redeclarable_node!(link => pub IntrusiveTest[field1: u8, field2: &'c str]: 'c);

  #[test]
  fn intrusive_redeclarable() {
    let datas = [(0, "a"), (1, "b"), (2, "c")];
    let bump = Arena::new();

    let head = IntrusiveTest::new_canonical(&bump, datas[0].0, datas[0].1);
    assert_eq!(head.field1, datas[0].0);
    assert_eq!(head.field2, datas[0].1);

    assert_node!(head.canonical(), head);
    assert_node!(head.latest(), head);
    assert!(head.prev().is_none());
    assert!(head.is_canonical());
    assert!(head.is_latest());
    unsafe {
      assert_node!(head.link.prev_or_latest.get().as_ref(), head);
      assert_node!(head.link.canonical.as_ref(), head);
    }

    let second = IntrusiveTest::new_node(&bump, head, datas[1].0, datas[1].1);
    assert_eq!(second.field1, datas[1].0);
    assert_eq!(second.field2, datas[1].1);

    assert_node!(second.canonical(), head);
    assert_node!(second.latest(), second);
    assert_node!(second.prev().unwrap(), head);
    assert!(!second.is_canonical());
    assert!(second.is_latest());
    unsafe {
      assert_node!(second.link.prev_or_latest.get().as_ref(), head);
      assert_node!(second.link.canonical.as_ref(), head);
    }

    assert_node!(head.canonical(), head);
    assert_node!(head.latest(), second);
    assert!(head.prev().is_none());
    assert!(head.is_canonical());
    assert!(!head.is_latest());
    unsafe {
      assert_node!(head.link.prev_or_latest.get().as_ref(), second);
      assert_node!(head.link.canonical.as_ref(), head);
    }

    let third = IntrusiveTest::new(&bump, Some(second), datas[2].0, datas[2].1);
    assert_eq!(third.field1, datas[2].0);
    assert_eq!(third.field2, datas[2].1);

    assert_node!(third.canonical(), head);
    assert_node!(third.latest(), third);
    assert_node!(third.prev().unwrap(), second);
    assert!(!third.is_canonical());
    assert!(third.is_latest());
    unsafe {
      assert_node!(third.link.prev_or_latest.get().as_ref(), second);
      assert_node!(third.link.canonical.as_ref(), head);
    }
    assert_node!(second.canonical(), head);
    assert_node!(second.latest(), third);
    assert_node!(second.prev().unwrap(), head);
    assert!(!second.is_canonical());
    assert!(!second.is_latest());
    unsafe {
      assert_node!(second.link.prev_or_latest.get().as_ref(), head);
      assert_node!(second.link.canonical.as_ref(), head);
    }
    assert_node!(head.canonical(), head);
    assert_node!(head.latest(), third);
    assert!(head.prev().is_none());
    assert!(head.is_canonical());
    assert!(!head.is_latest());
    unsafe {
      assert_node!(head.link.prev_or_latest.get().as_ref(), third);
      assert_node!(head.link.canonical.as_ref(), head);
    }

    let mut iter = third.iter();
    assert_eq!(iter.next().unwrap().field1, datas[2].0);
    assert_eq!(iter.next().unwrap().field1, datas[1].0);
    assert_eq!(iter.next().unwrap().field1, datas[0].0);
    assert!(iter.next().is_none());
    assert!(iter.next().is_none());
  }
}
