//! These 4 meta specs resembles those in
//! [`llvm::Triple`](https://github.com/llvm/llvm-project/blob/c71780cf5879e0070bc40e4fd236ced93b2f50b8/llvm/include/llvm/TargetParser/Triple.h#L47),
//! Although I intend to keep my compilation pipeline simple
//! but in order to emit llvm-compatible IR they are necessary.
#![allow(clippy::upper_case_acronyms)]
#![allow(non_camel_case_types)]

use ::rcc_adt::{Signedness, Size};
use ::rcc_utils::{ensure_is_pod, static_assert};

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
#[strum(serialize_all = "lowercase")]
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
#[strum(serialize_all = "lowercase")]
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
#[strum(serialize_all = "lowercase")]
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
#[strum(serialize_all = "lowercase")]
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
///
/// Currently this is only be hold inside ast/ir sessions, and the module.
#[derive(Debug, Default, Clone, Copy)]
pub struct Triple {
  pub architecture: Architecture,
  pub vendor: Vendor,
  pub operating_system: OperatingSystem,
  pub environment: Environment,
  pub object_format: ObjectFormat,
}

ensure_is_pod!(Triple);
static_assert!(::std::mem::size_of::<Triple>() <= 8, "Triple too large!");

#[derive(
  Debug,
  Clone,
  Copy,
  PartialEq,
  Eq,
  ::strum_macros::Display,
  ::strum_macros::EnumString,
  ::strum_macros::IntoStaticStr,
)]
pub enum CallingConvention {
  SystemV,
  /// Windows Fastcall.
  ///
  /// [x86 Calling convention](https://en.wikipedia.org/wiki/X86%20calling%20conventions)
  ///
  /// todo: how about those `__cdecl`, `__thiscall` and `__stdcall` shinanigans?
  Fastcall,
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
pub enum Endianess {
  #[default]
  Little,
  /// unsupported. just keep it to satisfy perfectionists like myself. :)
  Big,
}

/// TODO: this is used for IR and backend, so should not put it here maybe.
#[derive(
  Debug,
  Clone,
  Copy,
  PartialEq,
  Eq,
  ::strum_macros::Display,
  ::strum_macros::EnumString,
  ::strum_macros::IntoStaticStr,
)]
pub enum DataModel {
  /// (mostly) Win64
  ///
  /// `long long`, and `pointer` are 64 bits.
  LLP64,
  /// (mostly) 64-bit Unix systems
  ///
  /// `long`, and `pointer` are 64 bits.
  LP64,
  // ignoring LP32, ILP32, ILP64, etc.
}

impl Architecture {
  pub const fn endianness(self) -> Option<Endianess> {
    use Architecture::*;
    use Endianess::*;
    match self {
      RiscV64 | AArch64 | x86_64 => Some(Little),
      _ => None,
    }
  }

  pub const fn pointer_width(self) -> Option<Size> {
    use Architecture::*;
    match self {
      RiscV64 | AArch64 | x86_64 => Some(Size::U64),
      _ => None,
    }
  }
}
impl Architecture {
  pub const HOST: Self = {
    use ::std::env::consts::ARCH;
    use Architecture::*;
    match ARCH {
      "riscv64gc" => RiscV64,
      "aarch64" => AArch64,
      "x86_64" => x86_64,
      _ => Unknown,
    }
  };
}
impl OperatingSystem {
  pub fn calling_convention(self) -> Option<CallingConvention> {
    use CallingConvention::*;
    use OperatingSystem::*;
    match self {
      Linux => Some(SystemV),
      Windows => Some(Fastcall),
      _ => None,
    }
  }
}
impl OperatingSystem {
  pub const HOST: Self = {
    use ::std::env::consts::OS;
    use OperatingSystem::*;
    match OS {
      "linux" => Linux,
      "windows" => Windows,
      _ => Unknown,
    }
  };
}
impl Vendor {
  pub const HOST: Self = {
    use ::std::env::consts::OS;
    use Vendor::*;
    match OS {
      "linux" => Unknown,
      "windows" => PC,
      _ => Unknown,
    }
  };
}
impl Environment {
  pub const HOST: Self = {
    use ::std::env::consts::OS;
    use Environment::*;
    match OS {
      "linux" => GNU,
      "windows" => MSVC,
      _ => Unknown,
    }
  };
}
impl ObjectFormat {
  pub const HOST: Self = {
    use ::std::env::consts::OS;
    use ObjectFormat::*;
    match OS {
      "linux" => ELF,
      "windows" => COFF,
      _ => Unknown,
    }
  };
}
impl Triple {
  pub fn unknown() -> Self {
    Default::default()
  }
}
impl Triple {
  pub const HOST: Self = Self {
    architecture: Architecture::HOST,
    vendor: Vendor::HOST,
    operating_system: OperatingSystem::HOST,
    environment: Environment::HOST,
    object_format: ObjectFormat::HOST,
  };
}
impl Triple {
  pub const fn endianness(&self) -> Option<Endianess> {
    self.architecture.endianness()
  }

  pub const fn data_model(&self) -> Option<DataModel> {
    use DataModel::*;
    use OperatingSystem::*;
    let Some(pointer_width) = self.architecture.pointer_width() else {
      return None;
    };
    match (pointer_width, self.operating_system, self.architecture) {
      (Size::U64, Windows, _) => Some(LLP64),
      (Size::U64, Linux, _) => Some(LP64),
      _ => None,
    }
  }
}
impl CallingConvention {}
impl Endianess {}

impl DataModel {
  pub const fn char_signess(self) -> Signedness {
    use DataModel::*;
    use Signedness::*;

    match self {
      LLP64 | LP64 => Unsigned,
    }
  }

  pub const fn pointer_width(self) -> Size {
    use DataModel::*;

    match self {
      LLP64 | LP64 => Size::U64,
    }
  }

  pub const fn char_size(self) -> Size {
    use DataModel::*;

    match self {
      LLP64 | LP64 => Size::U8,
    }
  }

  pub const fn short_size(self) -> Size {
    use DataModel::*;

    match self {
      LLP64 | LP64 => Size::U16,
    }
  }

  pub const fn int_size(self) -> Size {
    use DataModel::*;

    match self {
      LLP64 | LP64 => Size::U32,
    }
  }

  pub const fn long_size(self) -> Size {
    use DataModel::*;

    match self {
      LLP64 => Size::U32,
      LP64 => Size::U64,
    }
  }

  pub const fn long_long_size(self) -> Size {
    use DataModel::*;

    match self {
      LLP64 | LP64 => Size::U64,
    }
  }

  pub const fn float_size(self) -> Size {
    use DataModel::*;

    match self {
      LLP64 | LP64 => Size::U32,
    }
  }

  pub const fn double_size(self) -> Size {
    use DataModel::*;

    match self {
      LLP64 | LP64 => Size::U64,
    }
  }

  pub const fn long_double_size(self) -> Size {
    use DataModel::*;
    match self {
      LLP64 => Size::U64,
      LP64 => Size::U128,
    }
  }
}

mod fmt {
  use ::std::fmt::{self, Display};

  use super::*;
  impl Display for Triple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(
        f,
        "{}-{}-{}-{}",
        self.architecture, self.vendor, self.operating_system, self.environment,
      )
    }
  }
}
#[cfg(test)]
mod tests {
  use ::std::env::consts::{
    ARCH, DLL_EXTENSION, DLL_PREFIX, DLL_SUFFIX, EXE_EXTENSION, EXE_SUFFIX,
    FAMILY, OS,
  };

  use super::*;
  #[test]
  fn basic_info() {
    println!("Architecture: {}", ARCH);
    println!("Family: {}", FAMILY);
    println!("OS: {}", OS);
    println!("DLL prefix: {}", DLL_PREFIX);
    println!("DLL suffix: {}", DLL_SUFFIX);
    println!("DLL extension: {}", DLL_EXTENSION);
    println!("EXE suffix: {}", EXE_SUFFIX);
    println!("EXE extension: {}", EXE_EXTENSION);

    println!("Host triple: {}", Triple::HOST);
    println!(
      "Host data model: {}",
      Triple::HOST
        .data_model()
        .expect("failed to get datamodel of host!")
    );
  }
}
