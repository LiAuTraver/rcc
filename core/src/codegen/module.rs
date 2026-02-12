#![allow(unused)]

use ::std::collections::LinkedList;
pub struct Module {}

pub struct Function {
  /// TODO: maybe use an **intrusive list** like ADT? That's how LLVM handles it to increase efficiency.
  blocks: LinkedList<BasicBlock>,
}

pub struct BasicBlock {}
