use ::rcc_utils::{Dummy, IntoWith};

use crate::{
  common::{Operator, SourceSpan},
  parser::expression::{Binary, ConstantLiteral, Expression},
};

impl<'context> Expression<'context> {
  pub fn oneplusone() -> Self {
    Binary {
      operator: Operator::Plus,
      left: Self::Constant(
        ConstantLiteral::Integral(1.into()).into_with(Dummy::dummy()),
      )
      .into(),
      right: Self::Constant(
        ConstantLiteral::Integral(1.into()).into_with(Dummy::dummy()),
      )
      .into(),
      span: SourceSpan::dummy(),
    }
    .into()
  }
}
