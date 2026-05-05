
// tentative
int ub2;
// definition
static int ub2 = 0;

// definition
int ub3 = 0;
// tentative
static int ub3;

// tentative
int undefined_behavior;
// tentative
static int undefined_behavior;

// tentative
static int y;
// declaration
extern int y;
// definition
static int y = 42;

// tentative
int z;
// definition
int z = 10;

// tentative
int t = 5;
// declaration
extern int t;

// declaration
extern int ub4;
// definition
int ub4 = 0;
// tentative
static int ub4;
// declaration
extern int ub4;

// `extern` here being ignored
extern int has_warning_ub5 = 0;
static int has_warning_ub5;

extern unsigned short truly_external;
extern unsigned short truly_external;

void use() {
  ub2 = 100;
  y = 50;
  has_warning_ub5 = 10;
  undefined_behavior = 90;
  truly_external = 100;
}