//! Initialization of objects.
//!
//! This module is responsible for processing initializers in variable declarations and static assertions.
//! It handles both scalar and aggregate initializers, including array initializers with designators.
//!
//! ### Implementation Note:
//! - C99 designated initializers and brace elision is one of the **most complex parts** I've ever implemented!!!
//! - The implementation is currently focused on correctness and clarity rather than performance.
//!   I know there's a lot of room for optimization, but I want to get it working first.
//!
//! ### todos:
//! - [`SourceSpan`] handling is very bad.
//! - Struct and union initializers are not implemented yet.
//! - VLA is treated as [`Incomplete`] and only allows empty initialization.
//!   it's simple here, but the upstream has blocked it as [`unimplemented!`].

use ::rcc_adt::Size;
use ::rcc_ast::types::{Array, ArraySize, QualifiedType, Type, TypeInfo};
use ::rcc_memory::{ArenaVec, BumpAllocator, CollectIn};
use ::rcc_parse::{declaration as pd, expression as pe};
use ::rcc_shared::{DiagData::*, SourceSpan};
use ::rcc_utils::RefEq;
use ::std::{collections::HashMap, ops::Deref};
use ArraySize::*;

use crate::{
  Sema,
  declaration::{self as sd, Index},
  expression as se,
  folding::Folder,
  semantics::HandleWith,
};

#[allow(non_upper_case_globals)]
const npos: Index = sd::Designator::npos;
#[allow(non_upper_case_globals)]
const zero: Index = Index::U0;
#[allow(non_upper_case_globals)]
const one: Index = Index::U8;
/// See the [Module level](self) for more information.
pub struct Initialization<'i, 'c>
where
  'c: 'i,
{
  sema: &'i Sema<'c>,
  /// Gloabl decl or local static decl
  requires_folding: bool,
}
impl<'i, 'c> Deref for Initialization<'i, 'c> {
  type Target = Sema<'c>;

  fn deref(&self) -> &'i Self::Target {
    self.sema
  }
}
#[derive(Debug)]
enum Kind {
  Implicit,
  Explicit,
}
use Kind::*;

impl Kind {
  #[inline(always)]
  fn is_implicit(&self) -> bool {
    matches!(self, Implicit)
  }
}

#[derive(Debug)]
enum InitNode<'c> {
  Scalar(pe::Expression<'c>),
  List(ArrayTree<'c>),
}
use InitNode::*;

impl InitNode<'_> {
  fn span(&self) -> SourceSpan {
    match self {
      Scalar(expr) => expr.span(),
      List(tree) => tree.span,
    }
  }
}

#[derive(Debug)]
struct TreeEntry<'c> {
  node: InitNode<'c>,
  index: Index,
  is_implicit: bool,
}

impl<'c> TreeEntry<'c> {
  #[inline]
  fn new(index: Index, node: InitNode<'c>, is_implicit: bool) -> Self {
    Self {
      index,
      node,
      is_implicit,
    }
  }
}

#[derive(Debug, Default)]
struct ArrayTree<'c> {
  entries: Vec<TreeEntry<'c>>,
  positions: HashMap<Index, Size>,
  max_index: Index,
  span: SourceSpan,
}

impl<'c> ArrayTree<'c> {
  #[inline]
  fn new(span: SourceSpan) -> Self {
    Self {
      span,
      ..Default::default()
    }
  }

  #[inline]
  fn is_empty(&self) -> bool {
    self.entries.is_empty()
  }

  #[inline]
  fn update_max_index(&mut self, index: Index) {
    if index == npos {
      return;
    }
    self.max_index = Ord::max(self.max_index, index);
  }

  fn insert_leaf(
    &mut self,
    index: Index,
    node: InitNode<'c>,
    is_implicit: bool,
  ) -> bool {
    self.update_max_index(index);
    if let Some(&pos) = self.positions.get(&index) {
      let entry = &mut self.entries[pos.get()];
      entry.node = node;
      entry.is_implicit = is_implicit;
      true
    } else {
      let pos = self.entries.len();
      self.positions.insert(index, pos.into());
      self.entries.push(TreeEntry::new(index, node, is_implicit));
      false
    }
  }

  fn insert_path(
    &mut self,
    path: &[Index],
    node: InitNode<'c>,
    is_implicit: bool,
  ) -> bool {
    if path.is_empty() {
      return false;
    }

    let index = path[0];
    if path.len() == 1 {
      return self.insert_leaf(index, node, is_implicit);
    }

    self.update_max_index(index);

    let pos = self.positions.get(&index).copied().unwrap_or_else(|| {
      let pos = self.entries.len().into();
      self.positions.insert(index, pos);
      self.entries.push(TreeEntry::new(
        index,
        List(ArrayTree::new(node.span())),
        is_implicit,
      ));
      pos
    });

    let entry = &mut self.entries[pos.get()];
    entry.is_implicit = is_implicit;

    let is_overridden = if !matches!(entry.node, List(_)) {
      entry.node = List(ArrayTree::new(node.span()));
      true
    } else {
      false
    };

    let deeper = match &mut entry.node {
      List(tree) => tree.insert_path(&path[1..], node, is_implicit),
      Scalar(_) => unreachable!(),
    };

    is_overridden || deeper
  }
}

/// Wrappers.
impl<'i, 'c> Initialization<'i, 'c> {
  pub fn new(sema: &'i Sema<'c>, requires_folding: bool) -> Self {
    Self {
      sema,
      requires_folding,
    }
  }

  pub fn doit(
    self,
    initializer: pd::Initializer<'c>,
    target_type: Option<QualifiedType<'c>>,
  ) -> (sd::Initializer<'c>, QualifiedType<'c>) {
    match initializer {
      pd::Initializer::Expression(expr) => self.top_scalar(expr, target_type),
      pd::Initializer::InitializerList(list) =>
        self.top_list(list, target_type),
    }
  }

  fn top_scalar(
    &self,
    expr: pe::Expression<'c>,
    target_type: Option<QualifiedType<'c>>,
  ) -> (sd::Initializer<'c>, QualifiedType<'c>) {
    let scalar =
      self.scalar(expr, target_type.unwrap_or(self.void_type().into()));
    let qualified_type = match target_type {
      None => *scalar.qualified_type(),
      Some(target_type) => self.complete_type_if_eligible(target_type, scalar),
    };

    (scalar.into(), qualified_type)
  }

  fn top_list(
    &self,
    list: pd::InitializerList<'c>,
    target_type: Option<QualifiedType<'c>>,
  ) -> (sd::Initializer<'c>, QualifiedType<'c>) {
    match target_type {
      Some(target_type) =>
        if target_type.has_vla_dim() && !list.entries.is_empty() {
          self.add_error(NonEmptyInitVLA, list.span);
          let list = sd::InitializerList::new(&[], list.span);
          (list.into(), target_type)
        } else {
          let (list, inferred_type) = self.list(list, target_type);
          (list.into(), inferred_type)
        },
      None => {
        self.add_error(DeducedTypeWithBracedInitializer, list.span);
        (self.__empty_expr.into(), self.void_type().into())
      },
    }
  }
}
/// Commons.
impl<'i, 'c> Initialization<'i, 'c> {
  fn list(
    &self,
    list: pd::InitializerList<'c>,
    target_type: QualifiedType<'c>,
  ) -> (sd::InitializerList<'c>, QualifiedType<'c>) {
    let pd::InitializerList { entries, span } = list;

    if target_type.is_scalar() {
      let (entries, _inferred) =
        self.array(entries, self.make_singleton_array_type(target_type));
      (sd::InitializerList::new(entries, span), target_type)
    } else if target_type.is_array() {
      let (entries, inferred_type) = self.array(entries, target_type);
      (sd::InitializerList::new(entries, span), inferred_type)
    } else {
      self.add_error(
        UnsupportedFeature(
          "Struct/union initializer not implemented yet".to_string(),
        ),
        span,
      );
      // ik the code here is a mess but idc this unimpltmtend amatch arm
      let pesudo_entry = self.arena().alloc([sd::InitializerListEntry::new(
        sd::Designator::Array(npos),
        sd::Initializer::Scalar(self.__empty_expr),
        true,
      )]);
      (sd::InitializerList::new(pesudo_entry, span), target_type)
    }
  }

  fn scalar(
    &self,
    expression: pe::Expression<'c>,
    target_type: QualifiedType<'c>,
  ) -> se::ExprRef<'c> {
    self
      .expression(expression)
      .map(|expr| self.assign_cvt_if_eligible(target_type, expr))
      .map(|expr| self.fold_if_eligible(expr))
      .handle_with(self, self.__empty_expr)
  }

  fn scalar_leaf_width(&self, target_type: QualifiedType<'c>) -> Size {
    match target_type.unqualified_type {
      Type::Array(array) => match array.size {
        Constant(size) =>
          size.saturating_mul(self.scalar_leaf_width(array.element_type)),
        Incomplete | Variable(_) => zero,
      },
      _ => one,
    }
  }

  fn make_singleton_array_type(
    &self,
    element_type: QualifiedType<'c>,
  ) -> QualifiedType<'c> {
    Type::Array(Array::new(element_type, ArraySize::ONE))
      .lookup(self)
      .into()
  }

  fn relative_scalar_path_from_flat(
    &self,
    mut object_type: QualifiedType<'c>,
    mut flat_index: Index,
  ) -> (Vec<Index>, QualifiedType<'c>) {
    let mut path = Vec::default();

    // struct/union unimplemented
    while let Type::Array(array) = object_type.unqualified_type {
      let stride = self.scalar_leaf_width(array.element_type);
      if stride == zero {
        // unknown downstream extent. keep this dimension at zero and let
        // deeper dimensions absorb the flat cursor for recovery.
        path.push(zero);
        object_type = array.element_type;
        continue;
      }

      let index = flat_index / stride;
      path.push(index);
      flat_index %= stride;
      object_type = array.element_type;
    }

    (path, object_type)
  }

  fn consume_object_initializer(
    &self,
    initializer: pd::Initializer<'c>,
    target_type: QualifiedType<'c>,
    object_path: Vec<Index>,
    state: &mut ArrayTree<'c>,
    kind: Kind,
  ) {
    match target_type.unqualified_type {
      Type::Array(_) => match initializer {
        pd::Initializer::InitializerList(list) => {
          let pd::InitializerList { entries, span } = list;
          let mut local_state = ArrayTree::new(span);
          self.consume_array_ilist(entries, target_type, &[], &mut local_state);
          self.record_array_node(state, object_path, List(local_state), kind);
        },
        pd::Initializer::Expression(expression) => {
          if Self::is_char_array_string_literal(&expression, target_type) {
            self.record_array_node(
              state,
              object_path,
              Scalar(expression),
              kind,
            );
            return;
          }

          // scalar-to-aggregate brace elision: initialize the first scalar leaf.
          let (mut rel_path, _) =
            self.relative_scalar_path_from_flat(target_type, zero);
          let mut full_path = object_path;
          full_path.append(&mut rel_path);
          self.record_array_node(state, full_path, Scalar(expression), kind);
        },
      },
      Type::Record(_) | Type::Union(_) => {
        self.add_error(
          UnsupportedFeature(
            "struct/union initializer not implemented yet".to_string(),
          ),
          initializer.span(),
        );

        match initializer {
          pd::Initializer::Expression(expression) =>
            self.record_array_node(state, object_path, Scalar(expression), kind),
          pd::Initializer::InitializerList(list) => {
            self.record_array_node(
              state,
              object_path,
              List(ArrayTree::new(list.span)),
              kind,
            );
          },
        }
      },
      _ => match initializer {
        pd::Initializer::Expression(expression) =>
          self.record_array_node(state, object_path, Scalar(expression), kind),
        pd::Initializer::InitializerList(list) => self.consume_braced_scalar(
          target_type,
          object_path,
          state,
          kind,
          list,
        ),
      },
    }
  }

  fn consume_braced_scalar(
    &self,
    target_type: QualifiedType<'c>,
    object_path: Vec<Index>,
    state: &mut ArrayTree<'c>,
    kind: Kind,
    list: pd::InitializerList<'c>,
  ) {
    self.add_warning(ExcessBraceAroundScalarInitializer, list.span);

    let mut local_state = ArrayTree::new(list.span);
    let pseudo_scalar = self.make_singleton_array_type(target_type);
    self.consume_array_ilist(
      list.entries,
      pseudo_scalar,
      &[],
      &mut local_state,
    );

    // zeroinit.
    if local_state.is_empty() {
      self.record_array_node(state, object_path, List(local_state), kind);
      return;
    }

    debug_assert!(
      local_state.entries.len() == 1,
      "excess elements shall be warned and ignored few lines above, so there \
       shall be at most one entry here."
    );

    // dont report error/warning(handled line up), just flatten this.
    let Some(entry) = local_state.entries.into_iter().next() else {
      return;
    };
    let TreeEntry {
      node,
      index,
      is_implicit,
    } = entry;

    debug_assert!(
      index == zero,
      "shall be handled just inside the ilist call few line above and \
       `assume`d it as zeroinit."
    );

    if !is_implicit {
      self.add_error(
        DesignatorForNonAggregate(target_type.to_string()),
        local_state.span,
      );
      // continue without recording this entry.
      return;
    }

    debug_assert!(
      !matches!(node, List(ref list) if !list.is_empty()),
      "shall be flattened recursively via the execution of this function \
       upstream... except it's zeroinit"
    );

    self.record_array_node(state, object_path, node, kind);
  }
}
/// Arrays.
impl<'i, 'c> Initialization<'i, 'c> {
  fn array(
    &self,
    entries: Vec<pd::InitializerListEntry<'c>>,
    array_type: QualifiedType<'c>,
  ) -> (&'c [sd::InitializerListEntry<'c>], QualifiedType<'c>) {
    let mut state = Default::default();
    self.consume_array_ilist(entries, array_type, &[], &mut state);

    let inferred_type = self.infer_array_type(array_type, &state);
    (
      self.materialize_array_entries(state, inferred_type),
      inferred_type,
    )
  }

  fn merge_array_types_max(
    &self,
    lhs: QualifiedType<'c>,
    rhs: QualifiedType<'c>,
  ) -> QualifiedType<'c> {
    let (Type::Array(lhs_array), Type::Array(rhs_array)) =
      (lhs.unqualified_type, rhs.unqualified_type)
    else {
      // can we reach here?
      return lhs;
    };

    let element_type = if lhs_array.element_type.is_array()
      && rhs_array.element_type.is_array()
    {
      self.merge_array_types_max(lhs_array.element_type, rhs_array.element_type)
    } else {
      lhs_array.element_type
    };

    let size = match (lhs_array.size, rhs_array.size) {
      (Constant(lhs), Constant(rhs)) => Constant(Ord::max(lhs, rhs)),
      (Constant(lhs), Incomplete | Variable(_)) => Constant(lhs),
      (Incomplete | Variable(_), Constant(rhs)) => Constant(rhs),
      (Incomplete | Variable(_), Incomplete | Variable(_)) => Incomplete,
    };

    Type::Array(Array::new(element_type, size))
      .lookup(self)
      .into()
  }

  fn resolve_array_designator_path(
    &self,
    designators: Vec<pd::Designator<'c>>,
    mut target_type: QualifiedType<'c>,
    span: SourceSpan,
  ) -> (Vec<Index>, QualifiedType<'c>) {
    let mut resolved = Vec::default();

    for designator in designators {
      match designator {
        pd::Designator::Index(expression) => {
          let index = self.try_fold_to_usize(expression, span);

          match target_type.unqualified_type {
            Type::Array(array) => {
              if let Constant(bound) = array.size
                && let Some(index) = index
                && index >= bound
              {
                self.add_error(
                  DesignatorIndexOutOfBound(index.get(), bound.get()),
                  span,
                );
              }
              resolved.push(index.unwrap_or(npos));
              target_type = array.element_type;
            },
            _ => {
              if let Some(index) = index {
                self.add_error(
                  Custom(format!(
                    "designator [{}] cannot be applied to non-array type '{}'",
                    index, target_type
                  )),
                  span,
                );
              }
              resolved.push(npos);
              break;
            },
          }
        },
        pd::Designator::Field(field) => {
          self.add_error(InvalidDesignator(true, field.to_string()), span);
          break;
        },
      }
    }

    (resolved, target_type)
  }

  fn consume_array_ilist(
    &self,
    entries: Vec<pd::InitializerListEntry<'c>>,
    array_type: QualifiedType<'c>,
    prefix: &[Index], // always empty for now, but leave rooms for extension...
    state: &mut ArrayTree<'c>,
  ) {
    let mut cursor_flat: Index = zero;
    let element_scalar_width = Ord::max(
      self.scalar_leaf_width(array_type.as_array_unchecked().element_type),
      one,
    );

    entries.into_iter().for_each(|entry| {
      self.consume_array_entry(
        array_type,
        prefix,
        state,
        &mut cursor_flat,
        element_scalar_width,
        entry,
      )
    });
  }

  fn consume_array_entry(
    &self,
    array_type: QualifiedType<'c>,
    prefix: &[Index],
    state: &mut ArrayTree<'c>,
    cursor_flat: &mut Index,
    element_scalar_width: Size,
    entry: pd::InitializerListEntry<'c>,
  ) {
    match entry {
      pd::InitializerListEntry::Initializer(initializer) => self
        .consume_anonymous_array_entry(
          array_type,
          prefix,
          state,
          element_scalar_width,
          cursor_flat,
          initializer,
        ),
      pd::InitializerListEntry::Designated(designated) => self
        .consume_designated_array_entry(
          array_type,
          prefix,
          state,
          element_scalar_width,
          cursor_flat,
          designated,
        ),
    }
  }

  fn consume_designated_array_entry(
    &self,
    array_type: QualifiedType<'c>,
    prefix: &[Index],
    state: &mut ArrayTree<'c>,
    element_scalar_width: Size,
    cursor_flat: &mut Index,
    designated: pd::Designated<'c>,
  ) {
    let pd::Designated {
      designators,
      initializer,
      span,
    } = designated;

    let (relative_path, designated_type) =
      self.resolve_array_designator_path(designators, array_type, span);

    if relative_path.is_empty() || relative_path.contains(&npos) {
      return;
    }
    let first_index = relative_path[0];

    debug_assert!(first_index != npos);

    *cursor_flat = first_index.saturating_mul(element_scalar_width);

    let mut object_path = prefix.to_owned();
    object_path.extend(relative_path);
    self.consume_object_initializer(
      initializer,
      designated_type,
      object_path,
      state,
      Explicit,
    );

    *cursor_flat = first_index
      .saturating_add(one)
      .saturating_mul(element_scalar_width);
  }

  fn consume_anonymous_array_entry(
    &self,
    array_type: QualifiedType<'c>,
    prefix: &[Index],
    state: &mut ArrayTree<'c>,
    element_scalar_width: Size,
    cursor_flat: &mut Index,
    initializer: pd::Initializer<'c>,
  ) {
    let element_type = array_type.as_array_unchecked().element_type;

    let array_bound = array_type.as_array_unchecked().size_opt();
    let total_scalars =
      array_bound.map(|bound| bound.saturating_mul(element_scalar_width));

    match initializer {
      pd::Initializer::InitializerList(ref list) => {
        let object_index = *cursor_flat / element_scalar_width;
        if let Some(bound) = array_bound
          && object_index >= bound
        {
          self.add_warning(ExcessElemInInitializer, list.span);
          *cursor_flat = cursor_flat.saturating_add(element_scalar_width);
          return;
        }

        let mut object_path = prefix.to_vec();
        object_path.push(object_index);
        self.consume_object_initializer(
          initializer,
          element_type,
          object_path,
          state,
          Implicit,
        );

        let next_object =
          (*cursor_flat / element_scalar_width).saturating_add(one);
        *cursor_flat = next_object.saturating_mul(element_scalar_width);
      },
      pd::Initializer::Expression(expression) => {
        if let Some(total) = total_scalars
          && *cursor_flat >= total
        {
          self.add_warning(ExcessElemInInitializer, expression.span());
          *cursor_flat = cursor_flat.saturating_add(one);
          return;
        }

        if element_type.is_array() {
          if Self::is_char_array_string_literal(&expression, element_type) {
            let object_index = *cursor_flat / element_scalar_width;
            if let Some(bound) = array_bound
              && object_index >= bound
            {
              self.add_warning(ExcessElemInInitializer, expression.span());
              *cursor_flat = cursor_flat.saturating_add(one);
              return;
            }

            let mut object_path = prefix.to_vec();
            object_path.push(object_index);
            self.consume_object_initializer(
              pd::Initializer::Expression(expression),
              element_type,
              object_path,
              state,
              Implicit,
            );

            *cursor_flat = object_index
              .saturating_add(one)
              .saturating_mul(element_scalar_width);
            return;
          }

          let mut rel_path = self
            .relative_scalar_path_from_flat(array_type, *cursor_flat)
            .0;
          let mut full_path = prefix.to_vec();
          full_path.append(&mut rel_path);
          self.record_array_node(
            state,
            full_path,
            Scalar(expression),
            Implicit,
          );
        } else {
          let object_index = *cursor_flat;
          let mut full_path = prefix.to_owned();
          full_path.push(object_index);
          self.record_array_node(
            state,
            full_path,
            Scalar(expression),
            Implicit,
          );
        }

        *cursor_flat = cursor_flat.saturating_add(one)
      },
    }
  }

  fn record_array_node(
    &self,
    state: &mut ArrayTree<'c>,
    path: Vec<Index>,
    node: InitNode<'c>,
    kind: Kind,
  ) {
    if path.is_empty() || path.contains(&npos) {
      return;
    }

    let span = node.span();

    if state.insert_path(&path, node, kind.is_implicit()) {
      /// Diag helper. render a designator path like `[0][2][3]`.
      fn render_array_path(path: &[Index]) -> String {
        debug_assert!(!path.is_empty());

        let mut rendered = String::with_capacity(path.len() * 4);

        use ::std::fmt::Write;
        let _idc_if_it_failed = path
          .iter()
          .try_for_each(|index| write!(rendered, "[{index}]"));
        rendered
      }
      self.add_warning(DuplicateInitializer(render_array_path(&path)), span);
    }
  }

  fn materialize_array_entries(
    &self,
    tree: ArrayTree<'c>,
    array_type: QualifiedType<'c>,
  ) -> &'c [sd::InitializerListEntry<'c>] {
    tree
      .entries
      .into_iter()
      .map(|entry| {
        sd::InitializerListEntry::new(
          sd::Designator::Array(entry.index),
          self.materialize_node(
            entry.node,
            array_type.as_array_unchecked().element_type,
          ),
          entry.is_implicit,
        )
      })
      .collect_in::<ArenaVec<_>>(self.arena())
      .into_bump_slice()
  }

  fn materialize_node(
    &self,
    node: InitNode<'c>,
    target_type: QualifiedType<'c>,
  ) -> sd::Initializer<'c> {
    match node {
      Scalar(expression) => self.scalar(expression, target_type).into(),
      List(tree) => {
        let list_target_type = if target_type.is_array() {
          target_type
        } else {
          self.make_singleton_array_type(target_type)
        };
        let span = tree.span;
        sd::InitializerList::new(
          self.materialize_array_entries(tree, list_target_type),
          span,
        )
        .into()
      },
    }
  }
}
/// array helpers.
impl<'i, 'c> Initialization<'i, 'c> {
  fn infer_array_type(
    &self,
    array_type: QualifiedType<'c>,
    tree: &ArrayTree<'c>,
  ) -> QualifiedType<'c> {
    let Type::Array(array) = array_type.unqualified_type else {
      return array_type;
    };

    let element_type = if array.element_type.is_array() {
      self.infer_child_element_type(array.element_type, tree)
    } else {
      array.element_type
    };

    let size = match array.size {
      Constant(size) => Constant(size),
      Incomplete | Variable(_) => Constant(tree.max_index.saturating_add(one)),
    };

    Type::Array(Array::new(element_type, size))
      .lookup(self)
      .into()
  }

  fn infer_child_element_type(
    &self,
    default_element_type: QualifiedType<'c>,
    tree: &ArrayTree<'c>,
  ) -> QualifiedType<'c> {
    let mut inferred = None;

    for entry in &tree.entries {
      if let List(child) = &entry.node {
        let child_inferred = self.infer_array_type(default_element_type, child);
        inferred = Some(match inferred {
          None => child_inferred,
          Some(prev) => self.merge_array_types_max(prev, child_inferred),
        });
      }
    }

    inferred.unwrap_or(default_element_type)
  }

  fn string_literal_len(expr: se::ExprRef<'c>) -> Option<Size> {
    expr
      .as_constant()
      .and_then(|constant| constant.as_string())
      .map(|&string| string.len().into())
  }

  fn is_char_array_string_literal(
    expression: &pe::Expression<'c>,
    target_type: QualifiedType<'c>,
  ) -> bool {
    target_type
      .as_array()
      .is_some_and(|arr| arr.element_type.is_character_type())
      && matches!(
        expression,
        pe::Expression::Constant(constant) if constant.inner.is_string()
      )
  }

  fn complete_type_if_eligible(
    &self,
    target_type: QualifiedType<'c>,
    expr: se::ExprRef<'c>,
  ) -> QualifiedType<'c> {
    let Type::Array(array) = target_type.unqualified_type else {
      return target_type;
    };

    if !matches!(array.size, Incomplete)
      || !array.element_type.is_character_type()
    {
      return target_type;
    }

    let Some(required_size) =
      Self::string_literal_len(expr).map(|len| len.saturating_add(one))
    else {
      return target_type;
    };

    Type::Array(Array::new(array.element_type, Constant(required_size)))
      .lookup(self)
      .into()
  }

  fn assign_cvt_if_eligible(
    &self,
    target_type: QualifiedType<'c>,
    expr: se::ExprRef<'c>,
  ) -> se::ExprRef<'c> {
    // NASTY EXCEPTION: character arrays initialized with strings
    if let Type::Array(array) = target_type.unqualified_type
      && array.element_type.is_character_type()
      && let Some(string_len) = Self::string_literal_len(expr)
    {
      if let Some(bound) = array.size_opt()
        && bound < string_len
      {
        self.add_error(
          Custom(format!(
            "initializer string requires array size at least {}, but target \
             size is {}",
            string_len, bound
          )),
          expr.span(),
        );
      }
      return expr;
    }

    let expr: se::ExprRef<'c> = expr.lvalue_conversion(self).decay(self);

    if RefEq::ref_eq(target_type.unqualified_type, self.void_type()) {
      expr
    } else {
      expr
        .assignment_conversion(self, &target_type)
        .handle_with(self, self.__empty_expr)
    }
  }

  fn fold_if_eligible(&self, expr: se::ExprRef<'c>) -> se::ExprRef<'c> {
    if !self.requires_folding {
      expr
    } else {
      Folder::new(self, false, expr).doit().unwrap_or_else(|| {
        // empty is error node currently
        self.add_error(
          ExprNotConstant(
            "Expression cannot be evaluated to a constant value".to_string(),
          ),
          expr.span(),
        );
        self.__empty_expr
      })
    }
  }

  fn try_fold_to_usize(
    &self,
    expression: pe::Expression<'c>,
    span: SourceSpan,
  ) -> Option<Size> {
    let analyzed = self
      .expression(expression)
      .handle_with(self, self.__empty_expr);

    if !analyzed.qualified_type().unqualified_type.is_integer() {
      self.add_error(
        NonIntegerInArraySubscript(analyzed.to_string()),
        analyzed.span(),
      );
      None?
    }

    match Folder::new(self, false, analyzed).doit() {
      Some(expr) => {
        if !expr.is_integer_constant() {
          self.add_error(
            ExprNotConstant(format!(
              "array designator index '{}' is not an integer constant \
               expression",
              expr
            )),
            expr.span(),
          );
          None?
        }

        match **expr {
          se::RawExpr::Constant(se::Constant::Integral(integral)) =>
            integral.try_into().ok().or_else(|| {
              self.add_error(
                DesignatorIndexNegative(integral.to_builtin()),
                span,
              );
              None
            }),
          _ => {
            self.add_error(
              NonIntegerInArraySubscript(expr.to_string()),
              expr.span(),
            );
            None
          },
        }
      },
      None => {
        self.add_error(
          ExprNotConstant(format!(
            "array designator index '{}' is not an integer constant expression",
            analyzed
          )),
          analyzed.span(),
        );
        None
      },
    }
  }
}
