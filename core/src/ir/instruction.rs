use ::rcc_utils::SmallString;

use crate::types::{Constant, QualifiedType};

#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
  /// A Virtual Register (vreg).
  ///
  /// Covers **both** user variables (`int x`) and compiler temps (`%1`).
  /// We use a usize ID because string lookups are slow in the backend.
  Reg(usize),

  /// A Global Label.
  ///
  /// This represents the **Address** of the global(i.e., [`Function`] and [`Variable`])
  /// Effectively a link-time constant.
  Label(SmallString),

  /// A Fixed Constyant (Immediate).
  Imm(Constant),
}

/// result = phi [val1, label1], [val2, label2]
#[derive(Debug, Clone)]
pub struct Phi {
  pub result: Operand, // The register defining the merged value
  pub incomings: Vec<(Operand, SmallString)>, // (Value, From_Block_Label)
}

#[derive(Debug)]
pub struct Jump {
  pub label: SmallString,
}
#[derive(Debug)]
pub struct Branch {
  pub cond: Operand,
  pub true_label: SmallString,
  pub false_label: SmallString,
}
#[derive(Debug)]
pub struct Return {
  pub returne: Option<Operand>,
}
#[derive(Debug)]
pub enum Terminator {
  /// Unconditional jump
  Jump(Jump),
  /// Conditional branch: if cond goto true_label else goto false_label
  Branch(Branch),
  /// Return from function
  Return(Return),
}

/// result = unary_op operand
#[derive(Debug)]
pub struct Unary<'context> {
  pub result: Operand,
  pub operator: UnaryOp,
  pub operand: Operand,
  pub qualified_type: QualifiedType<'context>,
}
#[derive(Debug)]
pub enum UnaryOp {
  Neg,
  Not,
  Compl,
}
/// result = binary_op lhs, rhs
#[derive(Debug)]
pub struct Binary<'context> {
  pub result: Operand,
  pub operator: BinaryOp,
  pub lhs: Operand,
  pub rhs: Operand,
  pub qualified_type: QualifiedType<'context>,
}
// arithematic ops only consider integer for now
#[derive(Debug, Clone, Copy, ::strum_macros::Display)]
pub enum BinaryOp {
  Add,
  Sub,
  Mul,
  Div,
  Mod,
  BitwiseAnd,
  BitwiseOr,
  BitwiseXor,
  LeftShift,
  RightShift,
}

#[derive(Debug)]
pub struct ICmp<'context> {
  pub result: Operand,
  pub predicate: ICmpPredicate,
  pub lhs: Operand,
  pub rhs: Operand,
  pub qualified_type: QualifiedType<'context>, // type of operands.
}
#[derive(Debug, Clone, Copy)]
pub enum ICmpPredicate {
  Eq,
  Ne,
  Slt,
  Sle,
  Sgt,
  Sge,
  Ult,
  Ule,
  Ugt,
  Uge,
}
/// Store value to address: *addr = value
#[derive(Debug)]
pub struct Store<'context> {
  pub addr: Operand,
  pub value: Operand,
  pub qualified_type: QualifiedType<'context>,
}

/// Load value from address: result = *addr
#[derive(Debug)]
pub struct Load<'context> {
  pub result: Operand,
  pub addr: Operand,
  pub qualified_type: QualifiedType<'context>,
}
#[derive(Debug)]
pub enum Memory<'context> {
  Store(Store<'context>),
  Load(Load<'context>),
  Alloca(Alloca<'context>),
}
/// Stack allocation.
/// result = alloca typeof(type)
/// Used for local variables that must live in memory (e.g., if their address is taken).
#[derive(Debug)]
pub struct Alloca<'context> {
  pub result: Operand,
  pub qualified_type: QualifiedType<'context>,
}

#[derive(Debug)]
pub enum Cast {
  // add later.
}

/// Function call: result = call func(args)
#[derive(Debug)]
pub struct Call {
  pub result: Option<Operand>,
  pub func: Operand,
  pub args: Vec<Operand>,
}

impl Call {
  pub fn new(
    result: Option<Operand>,
    func: Operand,
    args: Vec<Operand>,
  ) -> Self {
    Self { result, func, args }
  }
}

/// This mimics LLVM ir's catagory.
#[derive(Debug)]
pub enum Instruction<'context> {
  Phi(Phi),
  Terminator(Terminator),
  Unary(Unary<'context>),
  Binary(Binary<'context>),
  Memory(Memory<'context>),
  Cast(Cast),
  Call(Call),
  ICmp(ICmp<'context>),
  // etc...
}

::rcc_utils::interconvert!(Phi, Instruction<'context>);
::rcc_utils::interconvert!(Terminator, Instruction<'context>);
::rcc_utils::interconvert!(Unary, Instruction,'context);
::rcc_utils::interconvert!(Binary, Instruction,'context);
::rcc_utils::interconvert!(Memory, Instruction,'context);
::rcc_utils::interconvert!(Cast, Instruction<'context>);
::rcc_utils::interconvert!(Call, Instruction<'context>);
::rcc_utils::interconvert!(ICmp, Instruction,'context);
