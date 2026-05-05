// Non-static declaration follows static declaration
static int a;
int ub2 = 0;

// valid
static int b = 10;
extern int b;

// invalid, reason ditto
static int c = 10;
int c;

static int d;
// valid, with warning
extern int d = 10;
