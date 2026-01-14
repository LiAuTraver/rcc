pub mod debugging;

#[macro_export]
macro_rules! interconvert {
  ($inner:ident, $outer:ident) => {
    interconvert!($inner, $outer, $inner);
  };

  ($inner:ident, $outer:ident, $variant:ident) => {
    impl From<$inner> for $outer {
      fn from(value: $inner) -> Self {
        $outer::$variant(value)
      }
    }
    impl TryFrom<$outer> for $inner {
      type Error = ();

      fn try_from(value: $outer) -> Result<Self, Self::Error> {
        match value {
          $outer::$variant(inner) => Ok(inner),
          _ => Err(()),
        }
      }
    }
  };
}
