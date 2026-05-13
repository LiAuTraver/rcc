int y;
int y = 10;
int y;
const int A = 10;
const char B = A;
int putchar(int);
int puts(const char *);
int g(int);
extern int k;
int main() {
  putchar('H');
  char arr[6];
  arr[0] = 'H';
  arr[1] = 'e';
  arr[2] = 'l';
  arr[3] = 'l';
  arr[4] = 'o';
  arr[5] = '\0';
  puts(arr);
  // this failed rn:
  // puts("Hello");
  return k;
}

static const int C = 100;
int arr[C];

void f() { int arr2[C]; }
constexpr auto L3 = 5;
unsigned vla_fold[L3];