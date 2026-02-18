use ::rcc_utils::SmallString;
use ::std::collections::HashMap;

use super::{
  ilist_type,
  instruction::{self, Instruction, Operand},
  module::{self as ir, Module},
};
use crate::{
  analyzer::{
    declaration as ad,
    expression::{self as ae, ValueCategory},
    statement as astmt,
  },
  common::SourceSpan,
  session::Session,
  types::{Context, QualifiedType},
};

pub struct ModuleBuilder<'session, 'context, 'source>
where
  'context: 'session,
  'source: 'context,
{
  session: &'session Session<'context, 'source>,
  context: &'context Context<'context>,
  /// counter for generating unique temporary names
  temp_counter: usize,
  /// counter for generating unique label names
  label_counter: usize,
  /// The basic block currently being written into
  current_block: Option<ir::BasicBlock<'context>>,
  /// Blocks finalized in the current function
  current_blocks: ilist_type<ir::BasicBlock<'context>>,
  locals: HashMap<SmallString, Operand>,
}
impl<'session, 'context, 'source> ModuleBuilder<'session, 'context, 'source> {
  pub fn new(
    session: &'session Session<'context, 'source>,
    context: &'context Context<'context>,
  ) -> Self {
    Self {
      session,
      context,
      label_counter: 0,
      temp_counter: 0,
      current_block: None,
      current_blocks: Default::default(),
      locals: Default::default(),
    }
  }
}
impl<'session, 'context, 'source> ModuleBuilder<'session, 'context, 'source> {
  fn emit(&mut self, instruction: Instruction<'context>) {
    if let Some(block) = &mut self.current_block {
      block.instructions.push(instruction);
    }
  }

  fn push_block(&mut self, label: &str) {
    self.seal_current_block();
    self.current_block = Some(ir::BasicBlock {
      label: SmallString::from(label),
      instructions: ilist_type::new(),
    });
  }

  fn seal_current_block(&mut self) {
    if let Some(block) = self.current_block.take() {
      self.current_blocks.push(block);
    }
  }

  fn new_temp(&mut self) -> usize {
    self.temp_counter += 1;
    self.temp_counter
  }
}

impl<'session, 'context, 'source> ModuleBuilder<'session, 'context, 'source> {
  pub fn build(
    &mut self,
    translation_unit: ad::TranslationUnit<'context>,
  ) -> Module<'context> {
    let mut functions =
      ilist_type::with_capacity(translation_unit.declarations.len() * 2 / 3);
    let mut globals =
      Vec::with_capacity(translation_unit.declarations.len() / 3);
    translation_unit
      .declarations
      .into_iter()
      .for_each(|decl| match decl {
        ad::ExternalDeclaration::Function(function) =>
          functions.push(self.function(function)),
        ad::ExternalDeclaration::Variable(variable) =>
          globals.push(self.vardef(variable)),
      });
    Module { functions, globals }
  }

  pub fn function(
    &mut self,
    function: ad::Function<'context>,
  ) -> ir::Function<'context> {
    let ad::Function {
      symbol,
      parameters,
      specifier,
      body,
      labels,
      gotos,
      span,
    } = function;

    assert!(
      self.locals.is_empty() && self.current_block.is_none(),
      "not implemented for local func def!"
    );

    self.locals.clear();
    self.current_blocks = ilist_type::new();

    // bind parameters as operands
    let params: Vec<Operand> = parameters
      .into_iter()
      .map(|p| {
        let sym = p.symbol.borrow();
        let op = Operand::Reg(self.new_temp()); // adapt to your Operand variants
        self.locals.insert(sym.name.clone(), op.clone());
        op
      })
      .collect();

    if let Some(body) = body {
      self.compound(body);
      self.seal_current_block();
    }

    ir::Function {
      name: symbol.borrow().name.clone(),
      params,
      blocks: std::mem::take(&mut self.current_blocks),
      return_type: symbol
        .borrow()
        .qualified_type
        .as_functionproto_unchecked()
        .return_type,
      is_variadic: false,
    }
  }

  pub fn vardef(
    &mut self,
    variable: ad::VarDef<'context>,
  ) -> ir::Variable<'context> {
    todo!()
  }

  fn compound(&mut self, body: astmt::Compound) {
    let astmt::Compound { statements, span } = body;
    if statements.is_empty() {
      self.push_block("noop");
    }
    for statement in statements {
      self.statement(statement);
    }
    todo!("not implemented!!!")
  }

  fn statement(&mut self, statement: astmt::Statement) {
    match statement {
      astmt::Statement::Empty(_) => (),
      astmt::Statement::Return(return_stmt) => todo!(),
      astmt::Statement::Expression(expression) => todo!(),
      astmt::Statement::Declaration(external_declaration) => todo!(),
      astmt::Statement::Compound(compound) => todo!(),
      astmt::Statement::If(if_stmt) => todo!(),
      astmt::Statement::While(while_stmt) => todo!(),
      astmt::Statement::DoWhile(do_while) => todo!(),
      astmt::Statement::For(for_stmt) => todo!(),
      astmt::Statement::Switch(switch) => todo!(),
      astmt::Statement::Goto(goto) => todo!(),
      astmt::Statement::Label(label) => todo!(),
      astmt::Statement::Break(break_stmt) => todo!(),
      astmt::Statement::Continue(continue_stmt) => todo!(),
    }
  }
}
impl<'session, 'context, 'source> ModuleBuilder<'session, 'context, 'source> {
  fn expression(&mut self, expression: ae::Expression) -> Option<Operand> {
    let (raw_expr, qualified_type, value_category) = expression.destructure();
    match raw_expr {
      ae::RawExpr::Empty(_) => None,
      ae::RawExpr::Constant(constant) =>
        self.constant(constant, qualified_type, value_category),
      ae::RawExpr::Unary(unary) => todo!(),
      ae::RawExpr::Binary(binary) => todo!(),
      ae::RawExpr::Call(call) =>
        self.call(call, qualified_type, value_category),
      ae::RawExpr::Paren(paren) => todo!(),
      ae::RawExpr::MemberAccess(member_access) => todo!(),
      ae::RawExpr::Ternary(ternary) => todo!(),
      ae::RawExpr::SizeOf(size_of) => todo!(),
      ae::RawExpr::CStyleCast(cstyle_cast) => todo!(),
      ae::RawExpr::ArraySubscript(array_subscript) => todo!(),
      ae::RawExpr::CompoundLiteral(compound_literal) => todo!(),
      ae::RawExpr::Variable(variable) => todo!(),
      ae::RawExpr::ImplicitCast(implicit_cast) => todo!(),
      ae::RawExpr::Assignment(assignment) => todo!(),
    }
  }

  fn call(
    &mut self,
    call: ae::Call,
    qualified_type: QualifiedType,
    value_category: ValueCategory,
  ) -> Option<Operand> {
    let ae::Call {
      callee,
      arguments,
      span,
    } = call;
    let oprand_callee = self.expression(*callee);
    let oprand_args = arguments
      .into_iter()
      .map(|actual_parameter| {
        self.expression(actual_parameter).expect(
          "should have oprand (- RValue shall be handled in analyzer), or \
           probably unhandled situation",
        )
      })
      .collect();
    let retreg = if qualified_type.is_void() {
      None
    } else {
      Some(Operand::Reg(self.new_temp()))
    };
    self.emit(
      instruction::Call::new(
        retreg,
        todo!(
          "maybe refactor struct to directly contain a `module` so that we \
           can found the global funcion signature?"
        ),
        oprand_args,
      )
      .into(),
    );
    retreg
  }

  fn constant(
    &mut self,
    constant: ae::Constant,
    qualified_type: QualifiedType,
    value_category: ValueCategory, // should be RValue
  ) -> Option<Operand> {
    let ae::Constant { value, span } = constant;
    Some(Operand::Imm(value))
  }
}
