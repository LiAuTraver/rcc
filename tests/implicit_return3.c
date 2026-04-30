
// ret 0
int main(void) {
  if (0)
    int y = 10;
  else
    int k = 100;
}
// unreachable
int f(void) {
  if (0)
    int y = 10;
  else
    int k = 100;
}
