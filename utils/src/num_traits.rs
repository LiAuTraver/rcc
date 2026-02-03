macro_rules! traits {
  ($($variant:ty)*) => {
    ::paste::paste! {
        $(
            pub trait [< To $variant:camel >] {
                #[must_use]
                fn [< to_ $variant:lower >](self) -> $variant;
            }
        )*
    }
  };
}
macro_rules! impl_it {
    ($($t:ty)* : $variant:ty) => {
      ::paste::paste! {
          $(
              impl [<To $variant:camel>] for $t {
                  #[inline(always)]
                  fn [< to_ $variant:lower>](self) -> $variant {
                      self as $variant
                  }
              }
          )*
      }
    };
}

macro_rules! impl_all {
    ($($t:ty)*) => {
        // impl_it!($($t)* : bool);
        impl_it!($($t)* : i8);
        impl_it!($($t)* : i16);
        impl_it!($($t)* : i32);
        impl_it!($($t)* : i64);
        impl_it!($($t)* : i128);
        impl_it!($($t)* : u8);
        impl_it!($($t)* : u16);
        impl_it!($($t)* : u32);
        impl_it!($($t)* : u64);
        impl_it!($($t)* : u128);
        impl_it!($($t)* : isize);
        impl_it!($($t)* : usize);
    };
}

traits!(bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 isize usize);
impl_all!(bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 isize usize);
