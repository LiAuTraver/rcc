#[macro_export]
macro_rules! breakpoint {
  () => {{
    use std::io::{Write, stderr, stdout};
    std::intrinsics::breakpoint();
    _ = stdout().flush();
    _ = stderr().flush();
  }};
  ($($arg:tt)*) => {{
    use std::io::{Write, stderr, stdout};
    eprintln!("Fatal error:");
    eprintln!($($arg)*);
    _ = stdout().flush();
    _ = stderr().flush();
    std::intrinsics::breakpoint();
  }};
}
