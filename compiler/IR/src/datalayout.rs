
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
pub enum Endianess {
  #[default]
  Little,
  /// unsupported. just keep it to satisfy perfectionists like myself. :)
  Big,
}
