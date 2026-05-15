extern void truly_external();
int main(void);
void internal_or_ub() {}
void use();
int main(void);
static int redefined();
static void internal_or_ub();
void internal_or_ub();
extern void internal_or_ub();
void internal_or_ub();
static void internal_or_ub();
int redefined() { return 100; }

void use() {
  auto a = truly_external;
  auto b = internal_or_ub;
  auto c = redefined;
}
int main(void) {
  use();
  // extern int O;
  // // error
  // int O;
}

// int redefined() {}