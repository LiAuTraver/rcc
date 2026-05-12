use ::std::{
  cell::Cell,
  mem::{MaybeUninit, offset_of},
  ops::Deref,
  ptr::NonNull,
};

use super::fwd::Bump;

#[repr(C)]
#[derive(Debug)]
struct Link<T> {
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
  fn new(canonical: NonNull<T>, prev_or_latest: Cell<NonNull<T>>) -> Self {
    Self {
      canonical,
      prev_or_latest,
    }
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

  fn deref(&self) -> &Self::Target {
    &self.data
  }
}
::rcc_utils::static_assert!(
  offset_of!(Redeclarable<usize>, link) == 0,
  "repr(C) layout failed!"
);
::rcc_utils::static_assert_eq!(
  size_of::<Redeclarable<usize>>(),
  size_of::<Option<Redeclarable<usize>>>(),
  "[[no_unique_address]] failed!"
);
impl<T> Redeclarable<T> {
  fn new(
    canonical: NonNull<Self>,
    prev_or_latest: Cell<NonNull<Self>>,
    data: T,
  ) -> Self {
    Self {
      link: Link::new(canonical, prev_or_latest),
      data,
    }
  }

  pub fn new_node<'a>(bump: &'a Bump, prev: &Self, data: T) -> &'a Self {
    let canonical = prev.link.canonical;
    let prev = Cell::new(NonNull::from(prev));
    let this = bump.alloc(MaybeUninit::<Self>::uninit()).as_mut_ptr();
    unsafe {
      this.write(Redeclarable::new(canonical, prev, data));
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

  pub fn new_canonical(bump: &Bump, data: T) -> &Self {
    let this = bump.alloc(MaybeUninit::<Self>::uninit()).as_mut_ptr();
    unsafe {
      this.write(Redeclarable::new(
        NonNull::new_unchecked(this),
        Cell::new(NonNull::new_unchecked(this)),
        data,
      ));
    }
    unsafe { &*this }
  }
}

impl<T> Redeclarable<T> {
  pub fn canonical(&self) -> &Self {
    unsafe { self.link.canonical.as_ref() }
  }

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

  pub fn prev(&self) -> Option<&Self> {
    if ::std::ptr::eq::<Self>(self, self.link.canonical.as_ptr()) {
      None
    } else {
      unsafe { Some(self.link.prev_or_latest.get().as_ref()) }
    }
  }

  pub fn is_canonical(&self) -> bool {
    ::std::ptr::eq::<Self>(self, self.link.canonical.as_ptr())
  }

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

  #[test]
  fn t() {
    let datas = [0, 1, 2];
    let bump = Bump::new();

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
  }
}
