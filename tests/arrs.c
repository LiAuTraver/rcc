int a[100] = {1, 3, [20] = 4, 5, 7};

void f(int[]);
int main() {
  int b[100] = {1, 3, [20] = 4, 5, 7};
  auto c = b;

  int d[10][10] = {};
  f(a);
  f(b);
  f(c);
  f(d);
}