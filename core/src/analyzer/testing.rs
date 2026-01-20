use crate::{
  analyzer::expression::{Binary, Constant, Expression},
  common::operator::Operator,
  types::{Primitive, Type},
};

impl Expression {
  pub fn oneplusone() -> Self {
    Self::new_rvalue(
      Binary::new(
        Operator::Plus,
        Self::new_rvalue(
          Constant::Int(1).into(),
          Type::Primitive(Primitive::Int).into(),
        ),
        Self::new_rvalue(
          Constant::Int(1).into(),
          Type::Primitive(Primitive::Int).into(),
        ),
      )
      .into(),
      Type::Primitive(Primitive::Int).into(),
    )
  }
}
