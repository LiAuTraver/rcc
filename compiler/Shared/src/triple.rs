//! These 4 meta specs resembles those in [`llvm::Triple`](https://github.com/llvm/llvm-project/blob/c71780cf5879e0070bc40e4fd236ced93b2f50b8/llvm/include/llvm/TargetParser/Triple.h#L47),
//! I intend to keep my compilation pipeline simple
//! but in order to emit llvm-compatible IR they are necessary.
#![allow(clippy::upper_case_acronyms)]
#![allow(non_camel_case_types)]

#[derive(
  Debug,
  Clone,
  Copy,
  PartialEq,
  Eq,
  Default,
  ::strum_macros::Display,
  ::strum_macros::EnumString,
  ::strum_macros::IntoStaticStr,
)]
pub enum Architecture {
  #[default]
  Unknown,

  RiscV64,
  AArch64,
  x86_64,
}
#[derive(
  Debug,
  Clone,
  Copy,
  PartialEq,
  Eq,
  Default,
  ::strum_macros::Display,
  ::strum_macros::EnumString,
  ::strum_macros::IntoStaticStr,
)]
pub enum OperatingSystem {
  #[default]
  Unknown,

  Linux,
  Windows,
}
#[derive(
  Debug,
  Clone,
  Copy,
  PartialEq,
  Eq,
  Default,
  ::strum_macros::Display,
  ::strum_macros::EnumString,
  ::strum_macros::IntoStaticStr,
)]
pub enum Vendor {
  #[default]
  Unknown,

  PC,
}
#[derive(
  Debug,
  Clone,
  Copy,
  PartialEq,
  Eq,
  Default,
  ::strum_macros::Display,
  ::strum_macros::EnumString,
  ::strum_macros::IntoStaticStr,
)]
pub enum Environment {
  #[default]
  Unknown,

  GNU,
  LLVM,
  MSVC,
}
#[derive(
  Debug,
  Clone,
  Copy,
  PartialEq,
  Eq,
  Default,
  ::strum_macros::Display,
  ::strum_macros::EnumString,
  ::strum_macros::IntoStaticStr,
)]
pub enum ObjectFormat {
  #[default]
  Unknown,

  ELF,
  COFF,
}

/// Triple is probably not needed nor useful for my compilation pipeline;
/// see [module level documentation](self).
#[derive(Debug, Default)]
pub struct Triple {
  pub arch: Architecture,
  pub vendor: Vendor,
  pub os: OperatingSystem,
  pub env: Environment,
  pub objfmt: ObjectFormat,
}
