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

// tentative
int ub6;
// definition
static int ub6 = 0;

// definition
int ub3 = 0;
// tentative
static int ub3;

// tentative
int undefined_behavior;
// tentative
static int undefined_behavior;

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
