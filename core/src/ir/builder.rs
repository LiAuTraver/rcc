use ::rcc_utils::{SmallString, contract_violation};
use ::std::collections::HashMap;

use super::{
  ilist_type,
  instruction::{self as inst, Instruction, Operand},
  module::{self, BasicBlock, Module},
};
use crate::{
  common::{SourceSpan, StrRef},
  sema::{
    declaration as sd,
    expression::{self as se, ValueCategory},
    statement as ss,
  },
  session::Session,
  types::{Context, QualifiedType},
};

pub struct ModuleBuilder<'session, 'context, 'source>
where
  'context: 'session,
  'source: 'context,
{
  session: &'session Session<'context, 'source>,
  /// counter for generating unique temporary names
  temp_counter: usize,
  /// counter for generating unique label names
  label_counter: usize,
  /// The basic block currently being written into
  current_block: Option<BasicBlock<'context>>,
  /// Blocks finalized in the current function
  current_blocks: ilist_type<BasicBlock<'context>>,
  locals: HashMap<StrRef<'context>, Operand<'context>>,
  module: Module<'context>,
}
impl<'session, 'context, 'source> ModuleBuilder<'session, 'context, 'source> {
  pub fn new(session: &'session Session<'context, 'source>) -> Self {
    Self {
      session,
      label_counter: Default::default(),
      temp_counter: Default::default(),
      current_block: Default::default(),
      current_blocks: Default::default(),
      locals: Default::default(),
      module: Default::default(),
    }
  }
}
impl<'session, 'context, 'source> ModuleBuilder<'session, 'context, 'source> {
  fn emit(&mut self, instruction: Instruction<'context>) {
    if let Some(block) = &mut self.current_block {
      block.instructions.push(instruction);
      return;
    }
    panic!("no block to push.")
  }

  fn push_block(&mut self, label: &str) {
    self.seal_current_block();
    self.current_block = Some(BasicBlock {
      label: label.into(),
      instructions: ilist_type::new(),
    });
  }

  fn seal_current_block(&mut self) {
    if let Some(block) = self.current_block.take() {
      self.current_blocks.push(block);
    }
  }

  fn reg(&mut self) -> Operand<'context> {
    self.temp_counter += 1;
    Operand::Reg(self.temp_counter)
  }
}

impl<'session, 'context, 'source> ModuleBuilder<'session, 'context, 'source> {
  pub fn build(
    mut self,
    translation_unit: sd::TranslationUnit<'context>,
  ) -> Module<'context> {
    self.module.functions =
      ilist_type::with_capacity(translation_unit.declarations.len() * 2 / 3);
    self.module.globals =
      Vec::with_capacity(translation_unit.declarations.len() / 3);
    translation_unit
      .declarations
      .into_iter()
      .for_each(|decl| match decl {
        sd::ExternalDeclaration::Function(function) => {
          let function = self.function(function);
          self.module.functions.push(function)
        },
        sd::ExternalDeclaration::Variable(variable) => {
          let variable = self.vardef(variable);
          self.module.globals.push(variable)
        },
      });
    self.module
  }
}
impl<'session, 'context, 'source> ModuleBuilder<'session, 'context, 'source> {
  pub fn function(
    &mut self,
    function: sd::Function<'context>,
  ) -> module::Function<'context> {
    let sd::Function {
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
      "not implemented for local func decl!"
    );

    self.locals.clear();
    self.current_blocks = ilist_type::new();

    // bind parameters as operands
    let params: Vec<Operand> = parameters
      .into_iter()
      .map(|p| {
        let sym = p.symbol.borrow();
        let op = self.reg();
        self.locals.insert(sym.name, op.clone());
        op
      })
      .collect();

    if let Some(body) = body {
      self.compound(body);
      self.seal_current_block();
    }
    // locals clear, mamtake would take care of cur blocks
    self.locals.clear();

    module::Function {
      name: symbol.borrow().name,
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
    variable: sd::VarDef<'context>,
  ) -> module::Variable<'context> {
    todo!()
  }
}
impl<'session, 'context, 'source> ModuleBuilder<'session, 'context, 'source> {
  fn statement(&mut self, statement: ss::Statement<'context>) {
    #[allow(clippy::upper_case_acronyms)]
    type STMT<'a> = ss::Statement<'a>;
    match statement {
      STMT::Empty(_) => (),
      STMT::Return(return_stmt) => self.returnstmt(return_stmt),
      STMT::Expression(expression) => self.exprstmt(expression),
      STMT::Declaration(external_declaration) => todo!(),
      STMT::Compound(compound) => todo!(),
      STMT::If(if_stmt) => todo!(),
      STMT::While(while_stmt) => todo!(),
      STMT::DoWhile(do_while) => todo!(),
      STMT::For(for_stmt) => todo!(),
      STMT::Switch(switch) => todo!(),
      STMT::Goto(goto) => todo!(),
      STMT::Label(label) => todo!(),
      STMT::Break(break_stmt) => todo!(),
      STMT::Continue(continue_stmt) => todo!(),
    }
  }

  #[inline]
  fn exprstmt(&mut self, expression: se::Expression<'context>) {
    self.expression(expression);
  }

  fn returnstmt(&mut self, return_stmt: ss::Return<'context>) {
    let ss::Return { expression, span } = return_stmt;
    let operand = match expression {
      Some(expression) => self.expression(expression),
      None => None,
    };
    self.emit(Instruction::Terminator(inst::Terminator::Return(
      inst::Return { result: operand },
    )));
  }

  fn compound(&mut self, body: ss::Compound<'context>) {
    let ss::Compound { statements, span } = body;
    if statements.is_empty() {
      self.push_block("noop");
    } else {
      self.push_block("entry");
    }
    for statement in statements {
      self.statement(statement);
    }
  }
}
impl<'session, 'context, 'source> ModuleBuilder<'session, 'context, 'source> {
  fn expression(
    &mut self,
    expression: se::Expression<'context>,
  ) -> Option<Operand<'context>> {
    let (raw_expr, qualified_type, value_category) = expression
      .fold(&self.session.diagnosis)
      .unwrap()
      .destructure();
    #[allow(clippy::upper_case_acronyms)]
    type EXPR<'a> = se::RawExpr<'a>;
    match raw_expr {
      EXPR::Empty(_) => contract_violation!(
        "empty expr is used in sema for error recovery. shouldnt reach here."
      ),
      EXPR::Constant(constant) =>
        self.constant(constant, qualified_type, value_category),
      EXPR::Unary(unary) => todo!(),
      EXPR::Binary(binary) => todo!(),
      EXPR::Call(call) => self.call(call, qualified_type, value_category),
      EXPR::Paren(paren) => self.paren(paren, qualified_type, value_category),
      EXPR::MemberAccess(member_access) => todo!(),
      EXPR::Ternary(ternary) => todo!(),
      EXPR::SizeOf(size_of) => todo!(),
      EXPR::CStyleCast(cstyle_cast) => todo!(),
      EXPR::ArraySubscript(array_subscript) => todo!(),
      EXPR::CompoundLiteral(compound_literal) => todo!(),
      EXPR::Variable(variable) => todo!(),
      EXPR::ImplicitCast(implicit_cast) => todo!(),
      EXPR::Assignment(assignment) => todo!(),
    }
  }

  fn call(
    &mut self,
    call: se::Call<'context>,
    qualified_type: QualifiedType<'context>,
    value_category: ValueCategory,
  ) -> Option<Operand<'context>> {
    let se::Call {
      callee,
      arguments,
      span,
    } = call;
    // let callee_operand = self.expression(*callee);
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
      Some(self.reg())
    };

    let callee_name = callee
      .raw_expr()
      .as_variable()
      .expect("not implemented for complex callee!")
      .name
      .borrow()
      .name;

    let func_sig = self
      .module
      .functions
      .iter()
      .find(|f| ::std::ptr::eq(f.name, callee_name))
      .expect(
        "callee not yet emitted — maybe it's local funcdecl -- currently \
         havent handled it!",
      );

    self.emit(
      inst::Call::new(
        retreg.clone(),
        Operand::Label(func_sig.name),
        oprand_args,
      )
      .into(),
    );
    retreg
  }

  fn constant(
    &self,
    constant: se::Constant<'context>,
    qualified_type: QualifiedType<'context>,
    value_category: ValueCategory, // should be RValue
  ) -> Option<Operand<'context>> {
    debug_assert!(value_category == ValueCategory::RValue);
    Some(Operand::Imm(constant.value))
  }

  #[inline]
  fn paren(
    &mut self,
    paren: se::Paren<'context>,
    qualified_type: QualifiedType<'context>,
    value_category: ValueCategory,
  ) -> Option<Operand<'context>> {
    self.expression(*paren.expr)
  }
}
