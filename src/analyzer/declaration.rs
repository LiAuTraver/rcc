// placeholder
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Declaration;

mod fmt {
  use super::Declaration;
  use ::std::fmt::Display;

  impl Display for Declaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "<declaration>")
    }
  }
}
