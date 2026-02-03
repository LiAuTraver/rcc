use ::rc_utils::static_assert;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
  IEEE32 = 32, // standard 'float'
  IEEE64 = 64, // standard 'double'

               // IEEE128,      // 'long double' Quad precision
}

use Format::*;

type Underlying = u128;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Floating {
  bits: Underlying,
  format: Format,
}

static_assert!(
  ::std::mem::needs_drop::<Floating>() == false,
  "Floating should be a POD type without drop"
);

impl ::std::fmt::Display for Floating {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", self.bits)
  }
}

impl Floating {
  pub fn new<T: ::rc_utils::ToU128>(bits: T, format: Format) -> Self {
    Self {
      bits: bits.to_u128(),
      format,
    }
  }

  pub fn is_zero(&self) -> bool {
    match self.format {
      IEEE32 => f32::from_bits(self.bits as u32) == 0.0,
      IEEE64 => f64::from_bits(self.bits as u64) == 0.0,
    }
  }

  pub const fn format(&self) -> Format {
    self.format
  }
}

impl From<f32> for Floating {
  fn from(val: f32) -> Self {
    Floating::new(val.to_bits() as Underlying, IEEE32)
  }
}

impl From<f64> for Floating {
  fn from(val: f64) -> Self {
    Floating::new(val.to_bits() as Underlying, IEEE64)
  }
}

impl ::std::default::Default for Floating {
  fn default() -> Self {
    Self {
      bits: f64::default().to_bits() as Underlying,
      format: IEEE64,
    }
  }
}
