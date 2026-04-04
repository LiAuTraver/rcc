#![allow(unused)]

use ::rcc_ast::types::{self as ast, QualifiedType, TypeInfo};
use ::rcc_parse::{declaration as pd, expression as pe};
use ::rcc_shared::{ArenaVec, CollectIn, DiagData::*};
use ::std::ops::Deref;

use super::{declaration as sd, expression as se};
use crate::Sema;
pub struct Initialization<'i, 'c>
where
  'c: 'i,
{
  sema: &'i Sema<'c>,
  requires_folding: bool,
}
impl<'i, 'c> Deref for Initialization<'i, 'c> {
  type Target = Sema<'c>;

  fn deref(&self) -> &'i Self::Target {
    self.sema
  }
}
impl<'i, 'c> Initialization<'i, 'c> {
  pub fn new(sema: &'i Sema<'c>, requires_folding: bool) -> Self {
    Self {
      sema,
      requires_folding,
    }
  }

  pub fn doit(
    &self,
    initializer: pd::Initializer<'c>,
    target_type: Option<QualifiedType<'c>>,
  ) -> Option<sd::Initializer<'c>> {
    self.init(initializer, target_type)
  }

  fn init(
    &self,
    initializer: pd::Initializer<'c>,
    target_type: Option<QualifiedType<'c>>,
  ) -> Option<sd::Initializer<'c>> {
    match initializer {
      pd::Initializer::Expression(expression) => self
        .expression(expression)
        .map_err(|diag| self.add_diag(diag))
        .ok()
        .map(Into::into),
      pd::Initializer::InitializerList(list) =>
        self.list(list, target_type).map(Into::into),
    }
  }

  fn list(
    &self,
    list: pd::InitializerList<'c>,
    target_type: Option<QualifiedType<'c>>,
  ) -> Option<sd::InitializerList<'c>> {
    let pd::InitializerList { entries, span } = list;

    match target_type {
      Some(init_type) if init_type.unqualified_type.is_scalar() =>
        self.array(entries, target_type, ast::ArraySize::Constant(1)),
      Some(QualifiedType {
        unqualified_type: ast::Type::Array(array),
        ..
      }) => self.array(entries, Some(array.element_type), array.size),
      _ => todo!(),
    }
    .map(|entries| sd::InitializerList::new(entries, span))
  }

  fn array(
    &self,
    entries: Vec<pd::InitializerListEntry<'c>>,
    element_type: Option<QualifiedType<'c>>,
    size: ast::ArraySize,
  ) -> Option<&'c [sd::InitializerListEntry<'c>]> {
    let mut index: usize = 0;
    Some(
      entries
        .into_iter()
        .map(|entry| match entry {
          pd::InitializerListEntry::Designated(designated) => self
            .array_designated(designated, element_type, &mut index)
            .into(),
          pd::InitializerListEntry::Initializer(initializer) => self
            .array_annoymous(initializer, element_type, &mut index)
            .into(),
        })
        .collect_in::<ArenaVec<_>>(self.context().arena())
        .into_bump_slice(),
    )
  }

  fn array_annoymous(
    &self,
    initializer: pd::Initializer<'c>,
    target_type: Option<QualifiedType<'c>>,
    index: &mut usize,
  ) -> sd::InitializerEntry<'c> {
    let old = *index;
    *index += 1;
    self
      .init(initializer, target_type)
      .map(|initializer| {
        sd::InitializerEntry::new(sd::Designator::Array(old), initializer)
      })
      .expect("todo")
  }

  fn array_designated(
    &self,
    designated: pd::Designated<'c>,
    target_type: Option<QualifiedType<'c>>,
    index: &mut usize,
  ) -> sd::Designated<'c> {
    let pd::Designated {
      designators,
      initializer,
      span,
    } = designated;
    todo!()
  }

  fn scalar(
    &self,
    expression: pe::Expression<'c>,
    target_type: Option<QualifiedType<'c>>,
  ) -> Option<se::ExprRef<'c>> {
    self
      .expression(expression)
      .map(|expr| expr.lvalue_conversion(self.context()).decay(self.context()))
      .and_then(|expr| {
        if let Some(ref target_type) = target_type {
          expr.assignment_conversion(self.context(), target_type)
        } else {
          Ok(expr)
        }
      })
      .map(|expr| {
        let expression = if !self.requires_folding {
          expr
        } else {
          expr
            .fold(self.session)
            .inspect_error(|e| {
              self.add_error(
                ExprNotConstant(format!(
                  "Expression {e} cannot be evaluated to a constant value"
                )),
                e.span(),
              );
            })
            .take()
        };

        Some(expression)
      })
      .unwrap_or_else(|d| {
        self.add_diag(d);
        None
      })
  }
}
