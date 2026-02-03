use ::rc_utils::{Dummy, IntoWith};

use crate::{
  analyzer::expression::{Binary, ConstantLiteral, Expression},
  common::{Integral, Operator, SourceSpan},
  types::QualifiedType,
};

impl Expression {
  pub fn oneplusone() -> Self {
    Self::new_rvalue(
      Binary::new(
        Operator::Plus,
        Self::new_rvalue(
          ConstantLiteral::Integral(Integral::from_int(1))
            .into_with(Dummy::dummy()),
          QualifiedType::int(),
        ),
        Self::new_rvalue(
          ConstantLiteral::Integral(Integral::from_int(1))
            .into_with(Dummy::dummy()),
          QualifiedType::int(),
        ),
        SourceSpan::dummy(),
      )
      .into(),
      QualifiedType::int(),
    )
  }
}
