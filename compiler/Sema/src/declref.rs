use ::rcc_ast::{Context, VarDeclKind, types::QualifiedType};
use ::rcc_shared::Storage;
use ::rcc_utils::{PtrEq, StrRef};
use ::std::{cell::Cell, marker::PhantomData, ptr::NonNull};
use VarDeclKind::*;

#[derive(Debug)]
pub struct DeclNode<'c> {
  name: StrRef<'c>,
  qualified_type: QualifiedType<'c>,
  storage_class: Storage,
  /// the complex rule of [`Storage`] and [`VarDeclKind`] are managed by [`Linkage`].
  declkind: VarDeclKind,
  /// stores either the previous declaration or the latest declaration.
  ///
  /// - if the node is the canonical one, it points to the latest declaration in the chain, acts as a **infromation hub**;
  /// - otherwise, it points to the previous declaration.
  previous_or_latest_decl: Cell<DeclRef<'c>>,
  /// The nopde points to the eearlist appeared node.
  ///
  /// use as a **unique identifier** of this decl-linklist.
  canonical_decl: DeclRef<'c>,
}
/// SAFETY: this struct is safe as long as the [`DeclNode`] it points to are located inside the Arena, so does itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct DeclRef<'c> {
  ptr: NonNull<DeclNode<'c>>,
  marker: PhantomData<&'c DeclNode<'c>>,
}

impl<'c> DeclRef<'c> {
  pub fn new(
    context: &'c Context<'c>,
    qualified_type: QualifiedType<'c>,
    storage_class: Storage,
    name: StrRef<'c>,
    declkind: VarDeclKind,
    previous_decl: Option<DeclRef<'c>>,
  ) -> Self {
    use ::std::mem::MaybeUninit;

    let node_uninit =
      context.arena().alloc(MaybeUninit::<DeclNode<'_>>::uninit());

    let this = Self {
      ptr: unsafe { NonNull::new_unchecked(node_uninit.as_mut_ptr()) },
      marker: PhantomData,
    };
    let (canonical_decl, previous_or_latest_decl) =
      if let Some(prev) = previous_decl {
        prev
          .canonical_decl()
          .as_decl()
          .previous_or_latest_decl
          .set(this);
        (prev.canonical_decl(), prev.into())
      } else {
        (this, this.into())
      };
    node_uninit.write(DeclNode {
      name,
      qualified_type,
      storage_class,
      declkind,
      previous_or_latest_decl,
      canonical_decl,
    });

    this
  }

  #[inline]
  fn as_decl(self) -> &'c DeclNode<'c> {
    unsafe { self.ptr.as_ref() }
  }

  // /// UB.
  // #[inline]
  // fn as_decl_mut(self) -> &'c mut DeclNode<'c> {
  //   unsafe { &mut *self.ptr.as_ptr() }
  // }

  #[inline]
  pub fn qualified_type(self) -> QualifiedType<'c> {
    self.as_decl().qualified_type
  }

  #[inline]
  pub fn name(self) -> StrRef<'c> {
    self.as_decl().name
  }

  #[inline]
  pub fn storage_class(self) -> Storage {
    self.as_decl().storage_class
  }

  #[inline]
  pub fn declkind(self) -> VarDeclKind {
    self.as_decl().declkind
  }

  #[inline]
  pub fn previous_decl(self) -> Option<DeclRef<'c>> {
    if self.is_canonical() {
      None
    } else {
      Some(self.as_decl().previous_or_latest_decl.get())
    }
  }

  #[inline]
  pub fn canonical_decl(self) -> Self {
    self.as_decl().canonical_decl
  }

  /// Latest decl has the most information.
  #[inline]
  pub fn latest_decl(self) -> Self {
    self
      .canonical_decl()
      .as_decl()
      .previous_or_latest_decl
      .get()
  }

  /// Returns the definition [`DeclRef`] node if exists w.r.t. current symbol,
  /// otherwise returns [`None`].
  #[inline]
  pub fn definition(self) -> Option<DeclRef<'c>> {
    self.iter().find(|decl| decl.is_definition())
  }

  #[inline]
  pub fn iter(self) -> DeclIter<'c> {
    DeclIter::new(self)
  }

  #[inline]
  pub fn is_canonical(self) -> bool {
    PtrEq::ptr_eq(&self, &self.canonical_decl())
  }

  #[inline]
  pub fn is_latest(self) -> bool {
    PtrEq::ptr_eq(&self, &self.latest_decl())
  }

  #[inline]
  pub fn is_definition(self) -> bool {
    matches!(self.declkind(), Definition)
  }

  #[inline]
  pub fn is_typedef(self) -> bool {
    self.storage_class().is_typedef()
  }
}

pub struct DeclIter<'c> {
  current: DeclRef<'c>,
  /// Ensures stop after yielding the canonical node.
  done: bool,
}

impl<'c> DeclIter<'c> {
  pub fn new(current: DeclRef<'c>) -> Self {
    Self {
      current: current.latest_decl(),
      done: false,
    }
  }
}

impl<'c> Iterator for DeclIter<'c> {
  type Item = DeclRef<'c>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.done {
      None
    } else {
      let yielded = self.current;
      if self.current.is_canonical() {
        self.done = true;
      } else {
        self.current = self.current.as_decl().previous_or_latest_decl.get();
      }
      Some(yielded)
    }
  }
}

impl PtrEq for DeclRef<'_> {
  #[inline]
  fn ptr_eq(lhs: &Self, rhs: &Self) -> bool {
    lhs.ptr.as_ptr() == rhs.ptr.as_ptr()
  }
}

impl<'c> DeclRef<'c> {
  #[inline]
  pub fn decl(
    context: &'c Context<'c>,
    qualified_type: QualifiedType<'c>,
    storage_class: Storage,
    name: StrRef<'c>,
    previous_decl: Option<DeclRef<'c>>,
  ) -> Self {
    DeclRef::new(
      context,
      qualified_type,
      storage_class,
      name,
      Declaration,
      previous_decl,
    )
  }

  #[inline]
  pub fn def(
    context: &'c Context<'c>,
    qualified_type: QualifiedType<'c>,
    storage_class: Storage,
    name: StrRef<'c>,
    previous_decl: Option<DeclRef<'c>>,
  ) -> Self {
    DeclRef::new(
      context,
      qualified_type,
      storage_class,
      name,
      Definition,
      previous_decl,
    )
  }

  #[inline]
  pub fn tentative(
    context: &'c Context<'c>,
    qualified_type: QualifiedType<'c>,
    storage_class: Storage,
    name: StrRef<'c>,
    previous_decl: Option<DeclRef<'c>>,
  ) -> Self {
    DeclRef::new(
      context,
      qualified_type,
      storage_class,
      name,
      Tentative,
      previous_decl,
    )
  }
}
::rcc_utils::ensure_is_pod!(DeclNode<'_>);
::rcc_utils::ensure_is_pod!(DeclRef<'_>);

mod fmt {
  use ::std::fmt::{Display, Pointer};

  use super::*;

  impl<'c> Display for DeclNode<'c> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
      write!(f, "{}: {}", self.name, self.qualified_type)
    }
  }

  impl<'c> Pointer for DeclRef<'c> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{:p}", self.as_decl())
    }
  }

  impl<'c> Display for DeclRef<'c> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
      self.as_decl().fmt(f)
    }
  }
}
