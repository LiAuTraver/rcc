mod binary;
mod cast;
mod cmp;
mod memory;
mod misc;
mod terminator;

pub use self::{
  binary::{Binary, BinaryOp},
  cast::{
    BitCast, Cast, FPExt, FPToSI, FPToUI, FPTrunc, IntToPtr, PtrToInt, SIToFP,
    Sext, Trunc, UIToFP, Zext,
  },
  cmp::{Cmp, FCmp, FCmpPredicate, ICmp, ICmpPredicate},
  memory::{Alloca, Load, Memory, Store},
  misc::{Call, GetElementPtr, Phi, Select, Unary, UnaryOp},
  terminator::{Branch, Jump, Return, Terminator, Unreachable},
};
use super::ValueID;

pub trait User {
  fn use_list(&self) -> &[ValueID] {
    &[]
  }
}

/// This mimics LLVM ir's catagory.
#[derive(Debug)]
pub enum Instruction {
  Terminator(Terminator),
  Unary(Unary),
  Binary(Binary),
  Memory(Memory),
  Cast(Cast),
  Call(Call),
  Cmp(Cmp),
  Phi(Phi),
  Select(Select),
  GetElementPtr(GetElementPtr),
}
impl User for Instruction {
  fn use_list(&self) -> &[ValueID] {
    static_dispatch!(
      self,
      |variant| variant.use_list() =>
      Terminator Unary Binary Memory Cast Call Cmp Phi Select GetElementPtr
    )
  }
}

use ::rcc_utils::{interconvert, make_trio_for, static_dispatch};

interconvert!(Branch, Terminator);
interconvert!(Jump, Terminator);
interconvert!(Return, Terminator);
interconvert!(Unreachable, Terminator);

make_trio_for!(Branch, Terminator);
make_trio_for!(Jump, Terminator);
make_trio_for!(Return, Terminator);
make_trio_for!(Unreachable, Terminator);

interconvert!(Zext, Cast);
interconvert!(Sext, Cast);
interconvert!(Trunc, Cast);
interconvert!(FPExt, Cast);
interconvert!(FPTrunc, Cast);
interconvert!(FPToSI, Cast);
interconvert!(FPToUI, Cast);
interconvert!(UIToFP, Cast);
interconvert!(SIToFP, Cast);
interconvert!(PtrToInt, Cast);
interconvert!(IntToPtr, Cast);
interconvert!(BitCast, Cast);

interconvert!(Alloca, Memory);
interconvert!(Load, Memory);
interconvert!(Store, Memory);

interconvert!(ICmp, Cmp);
interconvert!(FCmp, Cmp);

interconvert!(Phi, Instruction);
interconvert!(Terminator, Instruction);
interconvert!(Unary, Instruction);
interconvert!(Binary, Instruction);
interconvert!(Memory, Instruction);
interconvert!(Cast, Instruction);
interconvert!(Call, Instruction);
interconvert!(Cmp, Instruction);
interconvert!(Select, Instruction);
interconvert!(GetElementPtr, Instruction);

make_trio_for!(Call, Instruction);
make_trio_for!(Phi, Instruction);
make_trio_for!(Terminator, Instruction);
make_trio_for!(Unary, Instruction);
make_trio_for!(Binary, Instruction);
make_trio_for!(Memory, Instruction);
make_trio_for!(Cast, Instruction);
make_trio_for!(Cmp, Instruction);
make_trio_for!(Select, Instruction);
make_trio_for!(GetElementPtr, Instruction);
