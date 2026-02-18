use ::rcc_utils::SmallString;

use super::instruction::{Instruction, Operand};
use crate::types::{Constant, QualifiedType};

/// keep it an alias type for latter convenience if choosing to optimize a lot
/// (e.g., switch to VecDeque, LinkedList or intrusive list. since now it's better for me to focus on the compiler design, not ADT.)
///
/// This name is from `llvm/ADT/ilist.h` and `llvm/ADT/ilist_node.h`, which is a doubly-linked intrusive list used to increase efficiency.
#[allow(non_camel_case_types)]
pub type ilist_type<T> = Vec<T>;

#[derive(Debug)]
pub struct Module<'context> {
  pub functions: ilist_type<Function<'context>>,
  pub globals: Vec<Variable<'context>>,
}

/// **Global** function in TAC-SSA form
#[derive(Debug)]
pub struct Function<'context> {
  pub name: SmallString,
  pub params: Vec<Operand>,
  pub blocks: ilist_type<BasicBlock<'context>>,
  pub return_type: QualifiedType<'context>,
  pub is_variadic: bool,
}

/// **Global** Variable. Non-static local variable won't be stored here, but exists as [`Operand`].
#[derive(Debug)]
pub struct Variable<'context> {
  pub name: SmallString,
  pub qualified_type: QualifiedType<'context>,
  pub initializer: Option<Initializer>,
}

#[derive(Debug)]
pub struct BasicBlock<'context> {
  pub label: SmallString,
  pub instructions: ilist_type<Instruction<'context>>,
}

/// **Static** initializer.
#[derive(Debug, Clone)]
pub enum Initializer {
  Scalar(Constant),
  Aggregate(Vec<Initializer>),
}
