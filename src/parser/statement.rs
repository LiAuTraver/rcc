use ::std::fmt::{Debug, Display};

use crate::parser::ast::Block;
use crate::parser::expression::Expression;

pub enum Statement {
  Return(Return),
  If(If),
}

pub struct Return {
  expression: Option<Expression>,
}

pub struct If {
  condition: Expression,
  if_branch: Block,
  else_branch: Block,
}

impl Display for Statement {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Statement::Return(ret) => <Return as Display>::fmt(ret, f),
      Statement::If(if_stmt) => <If as Display>::fmt(if_stmt, f),
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
      Some(expr) => write!(f, "return {:?}", expr),
      None => write!(f, "return"),
    }
  }
}

impl Debug for Return {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    <Self as Display>::fmt(self, f)
  }
}

impl Return {
  pub fn new(expression: Option<Expression>) -> Self {
    Self {
      expression: expression,
    }
  }
}

impl Display for If {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "if {:?} {:?}", self.condition, self.if_branch)?;
    if !self.else_branch.statements.is_empty() {
      write!(f, " else {:?}", self.else_branch)?;
    }
    Ok(())
  }
}
impl Debug for If {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    <Self as Display>::fmt(self, f)
  }
}
