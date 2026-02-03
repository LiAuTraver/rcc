use ::rc_utils::{Dummy, IntoWith};

use crate::{
  common::{Integral, Operator, SourceSpan},
  parser::expression::{Binary, ConstantLiteral, Expression},
};

impl Expression {
  pub fn oneplusone() -> Self {
    Self::Binary(Binary {
      operator: Operator::Plus,
      left: Self::Constant(
        ConstantLiteral::Integral(Integral::from_int(1))
          .into_with(Dummy::dummy()),
      )
      .into(),
      right: Self::Constant(
        ConstantLiteral::Integral(Integral::from_int(1))
          .into_with(Dummy::dummy()),
      )
      .into(),
      span: SourceSpan::dummy(),
    })
  }
}
