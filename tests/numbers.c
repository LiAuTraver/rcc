// this should pass; disambugate with octal numbers.
const auto _ = 0;
const auto _2 = 0u;

// all 42.
void int_test() {
  int a = 42;
  int b = -42;
  auto c = 42u;
  auto d = 42ul;
  auto e = 42ull;
}
// ditto
void bin_test() {
  int a = 0b101010;
  int b = -0b101010;
  auto c = 0b101010u;
  auto d = 0b101010ul;
  auto e = 0b101010ull;
}
// all 34
void oct_test() {
  int a = 042;
  int b = -042;
  // Octal literals without a '0o' prefix are deprecated.
  auto c = 0o42u;
  auto d = 0o42ul;
  auto e = 0o42ull;
  const auto _3 = 00u;
  const auto _4 = 07u;
}
// all 66
void hex_test() {
  int a = 0x42;
  int b = -0x42;
  auto c = 0x42u;
  auto d = 0x42ul;
  auto e = 0x42ull;
}
// all 42
void float_test() {
  float a = 42.0;
  double b = 42.0;
  auto c = 42.0f;
  auto d = 42.0l;
}

// 42 or 0.42
void nasty_float_test() {
  float a = .42;
  double b = 42.;
  auto c = .42f;
  auto d = 42.l;
}

void exponent_test() {
  float a = 42e10;
  double b = 42e-10;
  auto c = 42e+10f;
  auto d = 42E-10l;
  const auto e = 0x.42p10;
  const auto f = 0xe.42p10;
  const auto g = +0x0.42p10;
  const auto h = -0xABCDEF.42p+10;
  const auto i = -0x424242p-10;
  const auto j = 0x424242p-0;
  const auto k = 0x.0p+0;
  const auto l = 0x42E.0p+9l;
  const auto m = 0x.420p+0f;
}

// should pass
void negative_test() {
  int a = -42;
  auto b = -42u;
  auto c = -42ul;
  auto d = -42ull;
}
