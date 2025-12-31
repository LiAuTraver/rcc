use ::std::{
  cell::RefCell,
  collections::{HashMap, HashSet},
  rc::Rc,
};
#[cfg(test)]
use pretty_assertions::assert_eq;

use crate::common::{storage::Storage, types::QualifiedType};

/// as someone who came from C++, I'd more prefer to call it shared_ptr rather than Rc/RefCell or whatever. :p
#[allow(non_camel_case_types)]
pub type shared_ptr<T> = Rc<RefCell<T>>;
pub type SymbolRef = shared_ptr<Symbol>;

type ScopeAssoc<T> = HashMap<String, shared_ptr<T>>;
#[derive(Debug)]
pub struct Scope<T> {
  scopes: Vec<ScopeAssoc<T>>,
}
/// only tracks names
pub struct UnitScope {
  scopes: Vec<HashSet<String>>,
}
#[derive(Debug)]
pub struct Symbol {
  pub qualified_type: QualifiedType,
  pub storage_class: Storage,
  /// declaration or definition
  pub is_definition: bool,
  pub name: String,
}
#[derive(Debug)]
pub struct Environment {
  pub symbols: Scope<Symbol>,
}
impl Environment {
  pub fn new() -> Self {
    Self {
      symbols: Scope::new(),
    }
  }
  pub fn is_global(&self) -> bool {
    self.symbols.is_top_level()
  }
  pub fn enter(&mut self) {
    self.symbols.push_scope();
  }
  pub fn exit(&mut self) {
    self.symbols.pop_scope();
  }
  pub fn find(&self, name: &str) -> Option<shared_ptr<Symbol>> {
    self.symbols.get(name)
  }
}
impl Symbol {
  pub fn is_typedef(&self) -> bool {
    matches!(self.storage_class, Storage::Typedef)
  }
}
impl UnitScope {
  pub fn new() -> Self {
    Self { scopes: Vec::new() }
  }
  pub fn push_scope(&mut self) {
    self.scopes.push(HashSet::new());
  }
  pub fn pop_scope(&mut self) {
    self.scopes.pop();
  }
  pub fn shallow_contains(&self, name: &str) -> bool {
    self
      .scopes
      .last()
      .map_or(false, |scope| scope.contains(name))
  }
  pub fn contains(&self, name: &str) -> bool {
    for scope in self.scopes.iter().rev() {
      if scope.contains(name) {
        return true;
      }
    }
    false
  }
  pub fn declare(&mut self, name: String) {
    let current = self.scopes.last_mut();
    assert_eq!(
      current.is_some(),
      true,
      "No scope to declare variable `{}` in",
      name
    );
    current.unwrap().insert(name);
  }
  pub fn is_top_level(&self) -> bool {
    self.scopes.len() == 1
  }
}
impl<T> Scope<T> {
  pub fn new() -> Self {
    Self { scopes: Vec::new() }
  }
  pub fn push_scope(&mut self) {
    self.scopes.push(ScopeAssoc::new());
  }
  pub fn pop_scope(&mut self) {
    self.scopes.pop();
  }
  pub fn shallow_get(&self, name: &str) -> Option<shared_ptr<T>> {
    self
      .scopes
      .last()
      .and_then(|scope| scope.get(name).cloned())
  }
  pub fn get(&self, name: &str) -> Option<shared_ptr<T>> {
    for scope in self.scopes.iter().rev() {
      if let Some(val) = scope.get(name) {
        return Some(val.clone());
      }
    }
    None
  }
  pub fn declare(&mut self, name: String, val: shared_ptr<T>) -> shared_ptr<T> {
    let current = self.scopes.last_mut();
    assert_eq!(
      current.is_some(),
      true,
      "No scope to declare variable `{}` in",
      name
    );
    current.unwrap().insert(name, val.clone());
    val
  }
  pub fn is_top_level(&self) -> bool {
    self.scopes.len() == 1
  }
}
mod fmt {
  use super::Symbol;
  use ::std::fmt::Display;

  impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{}: {}", self.name, self.qualified_type)
    }
  }
}
