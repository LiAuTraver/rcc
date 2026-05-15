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

void f() {
  int arr2[C];
  {
    float C;
    double arr2;
    {
      extern const int C;
      auto external = C;
      extern int arr2[];
      auto e2 = arr2[0];
    }
  }
}
static constexpr const auto L3 = 5;
unsigned vla_fold[L3];