use ::rcc_ast::{Context, VarDeclKind, types::QualifiedType};
use ::rcc_shared::Storage;
use ::rcc_utils::{PtrEq, StrRef};
use ::std::{marker::PhantomData, ptr::NonNull};
use VarDeclKind::*;

#[derive(Debug)]
pub struct DeclNode<'c> {
  name: StrRef<'c>,
  qualified_type: QualifiedType<'c>,
  storage_class: Storage,
  /// for global variable, if the [`VarDeclKind`] is [`Definition`]
  /// and the [`Self::storage_class`] is [`Storage::Extern`], the [`Storage::Extern`] has no effect
  ///
  /// That being said, during TAC gen,
  /// - if the global vardef has both [`Storage::Extern`] and [`Definition`]
  ///   or one [`Tentative`] (one tantative counts as definition), add it as definition
  /// - else if only has [`Storage::Extern`] and [`Declaration`], it's declaration and let linker handle it.
  declkind: VarDeclKind,
  /// stores either the previous declaration or the latest declaration.
  ///
  /// - if the node is the canonical one, it points to the latest declaration in the chain;
  /// - otherwise, it points to the previous declaration.
  previous_or_latest_decl: DeclRef<'c>,
  /// The nopde points to the eearlist appeared node.
  canonical_decl: DeclRef<'c>,
  /// The node points to the `definition` node. This is not a good design, maybe fixme.
  ///
  /// # Directly access this field to judge whether a definition exists is wrong.
  /// only canonical one would be updated.
  definition: Option<DeclRef<'c>>,
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
    #[allow(clippy::uninit_assumed_init)]
    #[allow(invalid_value)]
    let node = context.arena().alloc(DeclNode {
      qualified_type,
      storage_class,
      name,
      declkind,
      canonical_decl: unsafe { MaybeUninit::uninit().assume_init() },
      previous_or_latest_decl: unsafe { MaybeUninit::uninit().assume_init() },
      definition: unsafe { MaybeUninit::uninit().assume_init() },
    });
    let this = Self {
      ptr: unsafe { NonNull::new_unchecked(&raw mut *node as *mut _) },
      marker: PhantomData,
    };

    (node.canonical_decl, node.previous_or_latest_decl) =
      if let Some(previous_decl) = previous_decl {
        previous_decl
          .canonical_decl()
          .as_decl_mut()
          .previous_or_latest_decl = this;
        (previous_decl.canonical_decl(), previous_decl)
      } else {
        (this, this)
      };
    node.definition = match declkind {
      Definition => {
        // if we are the definition, update the canonical node so all prior/future
        // nodes in the chain can find the definition.
        this.canonical_decl().set_definition(Some(this));
        Some(this)
      },
      _ => previous_decl.and_then(DeclRef::definition),
    };

    this
  }

  #[inline]
  fn as_decl(self) -> &'c DeclNode<'c> {
    unsafe { self.ptr.as_ref() }
  }

  #[inline]
  fn as_decl_mut(self) -> &'c mut DeclNode<'c> {
    unsafe { &mut *self.ptr.as_ptr() }
  }

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
      Some(self.as_decl().previous_or_latest_decl)
    }
  }

  #[inline]
  pub fn canonical_decl(self) -> DeclRef<'c> {
    self.as_decl().canonical_decl
  }

  #[inline]
  pub fn latest_decl(self) -> DeclRef<'c> {
    self.canonical_decl().as_decl().previous_or_latest_decl
  }

  /// Returns the definition [`DeclRef`] node if exists w.r.t. current symbol,
  /// otherwise returns [`None`].
  ///
  /// this it **NOT** a direct access to the `definition` field,
  /// but rather looking up through the canonical chain.
  ///
  /// To judge whether this particular declaration is a definition, use
  /// [`Self::declkind`] instead.
  #[inline]
  pub fn definition(self) -> Option<DeclRef<'c>> {
    self.canonical_decl().as_decl().definition
  }

  #[inline]
  pub fn is_canonical(self) -> bool {
    PtrEq::ptr_eq(&self, &self.canonical_decl())
  }

  #[inline]
  pub fn is_typedef(self) -> bool {
    self.storage_class().is_typedef()
  }

  /// tecnically speaking a node is created and shall never change except for the `definition` pointer,
  /// but here in my sema i didnt merge first then create node, but backpatching them, so here it serves as a workaround.
  #[inline]
  pub(super) fn refill(
    self,
    storage_class: Storage,
    qualified_type: QualifiedType<'c>,
  ) {
    let decl = self.as_decl_mut();
    decl.qualified_type = qualified_type;
    decl.storage_class = storage_class;
  }

  #[inline]
  fn set_definition(self, definition: Option<DeclRef<'c>>) {
    self.as_decl_mut().definition = definition;
  }
}

impl PtrEq for DeclRef<'_> {
  #[inline]
  fn ptr_eq(lhs: &Self, rhs: &Self) -> bool {
    lhs.ptr.as_ptr() == rhs.ptr.as_ptr()
  }
}

impl<'c> DeclNode<'c> {
  #[inline]
  pub fn decl(
    context: &'c Context<'c>,
    qualified_type: QualifiedType<'c>,
    storage_class: Storage,
    name: StrRef<'c>,
    previous_decl: Option<DeclRef<'c>>,
  ) -> DeclRef<'c> {
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
  ) -> DeclRef<'c> {
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
  ) -> DeclRef<'c> {
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
