
void overflow_test() {
  const int a = 2147483648;
  const int b = 2147483647 + 1;
  auto b = 4294967296u;
  auto c = 18446744073709551616ul;
  auto d = 18446744073709551616ull;
}

void invalid_suffix_test() {
  auto a = 42x;
  auto b = 42.0y;
}

void invalid_literal_test() {
  const auto _ = 08;
  const auto _3 = 0F;
  auto a = 0b102; // binary literals can only contain 0 and 1.
  auto b = 0o8;   // octal literals can only contain digits 0-7.
}

// exponent must be followed by a number.
void invalid_exponent_test() {
  auto a = 42e;
  auto b = 42e+;
  auto c = 42e-;
  auto d = 42E;
  auto e = 42p;
  // p cannot be used in a decimal float literal.
  auto f = 42p+10;
}

// hex floats
void invalid_hex_float_test() {
  auto a = 0x.42p; // must have an exponent digit.
  auto b = 0x.42p+;
  auto c = 0x.42p-;

  auto d = 0x.4210;      // no exp.
  auto e = 0x.124E2134;  // E cannot be used in a hex float literal.
  auto f = 0x.4210p01lr; // no exp.
}
