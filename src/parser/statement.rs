use crate::parser::declaration::{Block, VarDef};
use crate::parser::expression::Expression;

pub enum Statement {
  Empty(),
  Return(Return),
  If(If),
  // here only vardef, funcdef only permitted in top-level declarations hence it's handled there
  Declaration(VarDef),
  Expression(Expression),
  While(While),
  For(For),
  DoWhile(DoWhile),
  Switch(Switch),
  Case(Case),
  Default(Default),
  Label(Label),

  Break(),
  Continue(),
}

pub struct Return {
  pub expression: Option<Expression>,
}

pub struct If {
  pub condition: Expression,
  pub if_branch: Block,
  pub else_branch: Block,
}

pub struct While {
  pub condition: Expression,
  pub body: Block,
}
pub struct DoWhile {
  pub body: Block,
  pub condition: Expression,
}

pub struct For {
  pub initializer: Option<Box<Statement>>,
  pub condition: Option<Expression>,
  pub increment: Option<Expression>,
  pub body: Block,
}

pub struct Switch {
  pub expression: Expression,
  pub body: Box<Statement>, // Usually a Block
}

pub struct Case {
  pub value: Expression, // Must be constant integer expression
  pub body: Box<Statement>,
}

pub struct Default {
  pub body: Box<Statement>,
}

pub struct Label {
  pub name: String,
  pub statement: Box<Statement>,
}
impl Return {
  pub fn new(expression: Option<Expression>) -> Self {
    Self {
      expression: expression,
    }
  }
}
mod fmt {
  use crate::parser::{
    declaration::VarDef,
    expression::Expression,
    statement::{If, Return, Statement},
  };
  use std::fmt::{Debug, Display};
  impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
        Statement::Return(ret) => <Return as Display>::fmt(ret, f),
        Statement::If(if_stmt) => <If as Display>::fmt(if_stmt, f),
        Statement::Declaration(decl) => <VarDef as Display>::fmt(decl, f),
        Statement::Expression(expr) => <Expression as Display>::fmt(expr, f),
        Statement::Empty() => write!(f, ";"),
        _ => write!(f, "<unimplemented statement fmt>"),
      }
    }
  }

  impl Debug for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      <Self as Display>::fmt(self, f)
    }
  }

  impl Display for Return {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match &self.expression {
        Some(expr) => write!(f, "return {}", expr),
        None => write!(f, "return"),
      }
    }
  }

  impl Debug for Return {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      <Self as Display>::fmt(self, f)
    }
  }

  impl Display for If {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "if {} {}", self.condition, self.if_branch)?;
      if !self.else_branch.statements.is_empty() {
        write!(f, " else {}", self.else_branch)?;
      }
      Ok(())
    }
  }

  impl Debug for If {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      <Self as Display>::fmt(self, f)
    }
  }
}
