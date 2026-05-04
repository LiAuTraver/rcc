int main(void);
int main(void);
int main(void);
int main(void);
int main(void);
int main(void);
int main(void);
int main(void) {}

void f();
static void f();

int a;
static int a = 0;

int b = 0;
static int b;

extern int mystery[];
// fail
int mystery[sizeof(mystery) / sizeof(int) + 1];

int x;
static int x;

static int y;
extern int y;
int y = 42;

int z;
int z;
int z;
int z = 10;

int t = 5;
extern int t;
// fail
int t = 10;