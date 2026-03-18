int main() {
  int x;
  x = 0;
  // int y;
  // {
  //   int x;
  //   x = 2;
  //   y = !x;
  // }
  // x = 3;
  int *p;
  p = &x;
  x = !p;
  return 0;
}
