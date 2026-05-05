extern void truly_external();
int main(void);
void internal() {}
void use();
int main(void);
static int redefined();
static void internal();
void internal();
extern void internal();
void internal();
static void internal();
int redefined() { return 100; }

void use() {
  auto a = truly_external;
  auto b = internal;
  auto c = redefined;
}
int main(void) {
  use();
  extern int O;
  int O;
}

// int redefined() {}