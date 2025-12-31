use crate::{
  analyzer::{
    Analyzer,
    expression::{self as ae, ValueCategory},
  },
  common::{
    environment::Environment,
    error::Error,
    rawexpr::Unary,
    types::{Primitive, QualifiedType, Qualifiers, Type},
  },
  parser::{declaration as pd, expression as pe, statement as ps},
};

#[cfg(test)]
use ::pretty_assertions::assert_eq;

type ExprRes = Result<ae::Expression, Error>;

impl Analyzer {
  pub fn new(program: pd::Program) -> Self {
    Self {
      program,
      ..Analyzer::default()
    }
  }
  pub fn add_error(&mut self, error: String) {
    self.errors.push(error);
  }
  pub fn add_warning(&mut self, warning: String) {
    self.warnings.push(warning);
  }
  pub fn analyze(&mut self) {
    self.environment.enter();
    // todo!
    self.environment.exit();
  }
  pub fn errors(&self) -> &[String] {
    &self.errors
  }
  pub fn warnings(&self) -> &[String] {
    &self.warnings
  }
}

impl Analyzer {
  pub fn get_qualified_type(&mut self, declspecs: pd::DeclSpecs) -> QualifiedType {
    let unqualified_type = self.get_type(declspecs.type_specifiers);
    let qualifiers = declspecs.qualifiers;
    QualifiedType::new(qualifiers, unqualified_type)
  }
  fn get_type(&mut self, mut type_specifiers: Vec<pd::TypeSpecifier>) -> Type {
    assert_eq!(type_specifiers.is_empty(), false);
    // todo, convert typedefs into real types
    // type_specifiers.iter_mut().for_each(|ts| {});
    type_specifiers.sort_by_key(|s| s.sort_key());
    type Ts = pd::TypeSpecifier;
    // 6.7.3.1
    match type_specifiers.as_slice() {
      [Ts::Void] => Type::Primitive(Primitive::Void),
      [Ts::Char] => Type::Primitive(Primitive::Int8),
      [Ts::Signed, Ts::Char] => Type::Primitive(Primitive::Int8),
      [Ts::Unsigned, Ts::Char] => Type::Primitive(Primitive::Uint8),
      [Ts::Short]
      | [Ts::Short, Ts::Int]
      | [Ts::Signed, Ts::Short]
      | [Ts::Signed, Ts::Short, Ts::Int] => Type::Primitive(Primitive::Int16),
      [Ts::Unsigned, Ts::Short] | [Ts::Unsigned, Ts::Short, Ts::Int] => {
        Type::Primitive(Primitive::Uint16)
      }
      [Ts::Int] | [Ts::Signed] | [Ts::Signed, Ts::Int] => Type::Primitive(Primitive::Int32),
      [Ts::Unsigned] | [Ts::Unsigned, Ts::Int] => Type::Primitive(Primitive::Uint32),
      [Ts::Long]
      | [Ts::Long, Ts::Int]
      | [Ts::Signed, Ts::Long]
      | [Ts::Signed, Ts::Long, Ts::Int] => Type::Primitive(Primitive::Int64),
      [Ts::Unsigned, Ts::Long] | [Ts::Unsigned, Ts::Long, Ts::Int] => {
        Type::Primitive(Primitive::Uint64)
      }
      [Ts::Long, Ts::Long]
      | [Ts::Long, Ts::Long, Ts::Int]
      | [Ts::Signed, Ts::Long, Ts::Long]
      | [Ts::Signed, Ts::Long, Ts::Long, Ts::Int] => Type::Primitive(Primitive::Int64),
      [Ts::Unsigned, Ts::Long, Ts::Long] | [Ts::Unsigned, Ts::Long, Ts::Long, Ts::Int] => {
        Type::Primitive(Primitive::Uint64)
      }
      [Ts::Float] => Type::Primitive(Primitive::Float32),
      // treat long double as double for now
      [Ts::Double] | [Ts::Long, Ts::Double] => Type::Primitive(Primitive::Float64),
      [Ts::Float, Ts::Complex] => Type::Primitive(Primitive::Float32),
      [Ts::Double, Ts::Complex] | [Ts::Long, Ts::Double, Ts::Complex] => {
        Type::Primitive(Primitive::Float64)
      }
      [Ts::Bool] => Type::Primitive(Primitive::Bool),

      // skip _BitInt, _Decimal32, _Decimal64, _Decimal128 here
      _ => todo!("union, struct, enum, typedef, typeof, etc."),
    }
  }
}

impl Analyzer {
  fn analyze_expression(&mut self, expression: pe::Expression) -> ExprRes {
    match expression {
      pe::Expression::Empty => Ok(ae::Expression::default()),
      pe::Expression::Constant(constant) => self.analyze_constant(constant),
      pe::Expression::Unary(unary) => self.analyze_unary(unary),
      pe::Expression::Binary(binary) => self.analyze_binary(binary),
      pe::Expression::Assignment(assignment) => self.analyze_assignment(assignment),
      pe::Expression::Variable(variable) => self.analyze_variable(variable),
      pe::Expression::Call(call) => todo!(),
      pe::Expression::MemberAccess(member_access) => todo!(),
      pe::Expression::Ternary(ternary) => self.analyze_ternary(ternary),
      pe::Expression::SizeOf(size_of) => todo!(),
      pe::Expression::Cast(cast) => todo!(),
      pe::Expression::ArraySubscript(array_subscript) => todo!(),
      pe::Expression::CompoundLiteral(compound_literal) => todo!(),
    }
  }
  fn analyze_variable(&mut self, variable: pe::Variable) -> ExprRes {
    let symbol = self.environment.find(&variable.name).ok_or(())?;
    if symbol.borrow().is_typedef() {
      Err(())
    } else {
      Ok(ae::Expression::new(
        ae::RawExpr::Variable(ae::Variable::new(symbol.clone())),
        symbol.borrow().qualified_type.clone(),
        ValueCategory::LValue,
      ))
    }
  }
  fn analyze_constant(&mut self, constant: pe::Constant) -> ExprRes {
    let rawtype = match constant {
      ae::Constant::Int8(_) => Type::Primitive(Primitive::Int8),
      ae::Constant::Int16(_) => Type::Primitive(Primitive::Int16),
      ae::Constant::Int32(_) => Type::Primitive(Primitive::Int32),
      ae::Constant::Int64(_) => Type::Primitive(Primitive::Int64),
      ae::Constant::Uint8(_) => Type::Primitive(Primitive::Uint8),
      ae::Constant::Uint16(_) => Type::Primitive(Primitive::Uint16),
      ae::Constant::Uint32(_) => Type::Primitive(Primitive::Uint32),
      ae::Constant::Uint64(_) => Type::Primitive(Primitive::Uint64),
      ae::Constant::Float32(_) => Type::Primitive(Primitive::Float32),
      ae::Constant::Float64(_) => Type::Primitive(Primitive::Float64),
      ae::Constant::Bool(_) => Type::Primitive(Primitive::Bool),
      ae::Constant::String(_) => todo!(),
    };
    let qualty = QualifiedType::new(Qualifiers::empty(), rawtype);
    Ok(ae::Expression::new(
      ae::RawExpr::Constant(constant),
      qualty,
      ValueCategory::RValue,
    ))
  }
  fn analyze_unary(&mut self, unary: pe::Unary) -> ExprRes {
    let pe::Unary {
      operator,
      expression: pe_expr,
    } = unary;
    let expression = self.analyze_expression(*pe_expr)?;
    // TODO: type promotion of the unary and the expr_type
    let ty = expression.qualified_type().clone();
    let value_category = ValueCategory::RValue;
    Ok(ae::Expression::new(
      ae::RawExpr::Unary(Unary::new(operator, expression)),
      ty,
      value_category,
    ))
  }
  fn analyze_binary(&mut self, binary: pe::Binary) -> ExprRes {
    let pe::Binary {
      left: pe_left,
      operator,
      right: pe_right,
    } = binary;
    let left = self.analyze_expression(*pe_left)?;
    let right = self.analyze_expression(*pe_right)?;
    // ditto, todo
    let ty = left.qualified_type().clone();
    Ok(ae::Expression::new(
      ae::RawExpr::Binary(ae::Binary::new(operator, left, right)),
      ty,
      ValueCategory::RValue,
    ))
  }
  fn analyze_ternary(&mut self, ternary: pe::Ternary) -> ExprRes {
    let pe::Ternary {
      condition: pe_condition,
      if_branch: pe_then_expr,
      else_branch: pe_else_expr,
    } = ternary;
    let condition = self.analyze_expression(*pe_condition)?;
    let then_expr = self.analyze_expression(*pe_then_expr)?;
    let else_expr = self.analyze_expression(*pe_else_expr)?;
    // ditto, todo
    let ty = then_expr.qualified_type().clone();
    if then_expr.raw_type() != else_expr.raw_type() {
      Err(())
    } else {
      Ok(ae::Expression::new(
        ae::RawExpr::Ternary(ae::Ternary::new(condition, then_expr, else_expr)),
        ty,
        ValueCategory::RValue,
      ))
    }
  }
  fn analyze_assignment(&mut self, assignment: pe::Assignment) -> ExprRes {
    let pe::Assignment {
      left: pe_left,
      right: pe_right,
    } = assignment;
    let left = self.analyze_expression(*pe_left)?;
    let right = self.analyze_expression(*pe_right)?;
    if !left.is_modifiable_lvalue() {
      Err(())
    } else {
      todo!()
    }
  }
}
impl Analyzer {
  fn analyze_declaration(&mut self) {}
  fn analyze_function(&mut self, function: pd::Function) {
    let pd::Function {
      body,
      declarator,
      declspecs,
    } = function;
  }
  fn analyze_compound(&mut self, compound: ps::Compound) {}
}
impl ::core::default::Default for Analyzer {
  fn default() -> Self {
    Self {
      program: pd::Program::new(),
      environment: Environment::new(),
      errors: Vec::new(),
      warnings: Vec::new(),
    }
  }
}
mod test {

  #[test]
  fn oneplusone() {
    use crate::{analyzer::Analyzer, parser};
    // 1 + 1
    let mut analyzer = Analyzer::default();
    let expr = parser::expression::Expression::Binary(parser::expression::Binary {
      left: Box::new(parser::expression::Expression::Constant(
        crate::common::rawexpr::Constant::Int32(1),
      )),
      operator: crate::common::operator::Operator::Plus,
      right: Box::new(parser::expression::Expression::Constant(
        crate::common::rawexpr::Constant::Int32(1),
      )),
    });
    let analyzed_expr = analyzer.analyze_expression(expr);
    println!("{:#?}", analyzed_expr.unwrap());
  }
}
