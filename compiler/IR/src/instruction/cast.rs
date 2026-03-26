use ::rcc_utils::static_dispatch;

use super::User;
use crate::ValueID;
macro_rules! generate_cast_inst_for {
  ($(
    $(#[doc = $doc:expr])*
    $cast_inst:ident
  ),*) => {
    $(
      $(#[doc = $doc])*
      #[derive(Debug)]
      pub struct $cast_inst {
        operand: [ValueID; 1],
      }

      impl $cast_inst {
        pub fn new(operand: ValueID) -> Self {
          Self { operand: [operand] }
        }

        pub fn operand(&self) -> ValueID {
          self.operand[0]
        }
      }

      impl User for $cast_inst {
        fn use_list(&self) -> &[ValueID] {
          &self.operand
        }
      }
    )*
  };
}
macro_rules! impl_all {
  ($(
    $(#[doc = $doc:expr])*
    $cast_inst:ident
  ),*) => {
    $(
      generate_cast_inst_for!(
        $(#[doc = $doc])*
        $cast_inst
      );
    )*
  };
}

impl_all!(
  /// Integer truncation. The target width must be smaller than the operand.
  Trunc,
  /// Integer zero extension. The target width must be larger than the operand.
  Zext,
  /// Integer sign extension. The target width must be larger than the operand.
  Sext,
  /// Floating-point extension. The target width must be larger than the operand.
  FPExt,
  /// Floating-point truncation. The target width must be smaller than the operand.
  FPTrunc,
  /// Floating-point to unsigned integer.
  FPToUI,
  /// Floating-point to signed integer.
  FPToSI,
  /// Unsigned integer to floating-point.
  UIToFP,
  /// Signed integer to floating-point.
  SIToFP,
  /// Pointer to integer cast.
  PtrToInt,
  /// Integer to pointer cast.
  IntToPtr,
  /// Noop cast, just reinterpreting the bits.
  BitCast
);
#[derive(Debug)]
pub enum Cast {
  Trunc(Trunc),
  Zext(Zext),
  Sext(Sext),
  FPExt(FPExt),
  FPTrunc(FPTrunc),
  FPToUI(FPToUI),
  FPToSI(FPToSI),
  UIToFP(UIToFP),
  SIToFP(SIToFP),
  PtrToInt(PtrToInt),
  IntToPtr(IntToPtr),
  BitCast(BitCast),
}
impl User for Cast {
  fn use_list(&self) -> &[ValueID] {
    static_dispatch!(self, |variant| variant.use_list() =>
      Trunc Zext Sext FPExt FPTrunc FPToUI FPToSI UIToFP SIToFP PtrToInt IntToPtr BitCast
    )
  }
}
