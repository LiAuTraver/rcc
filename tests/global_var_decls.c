
// tentative
static int y_internal;
// declaration
extern int y_internal;
// definition
static int y_internal = 42;

// tentative
int z_external;
// definition
int z_external = 10;

// tentative
int t_external = 5;
// declaration
extern int t_external;

extern unsigned short truly_external;
extern unsigned short truly_external;

extern int k[];
// tentative array def of incomplete array assumed to have 1 elem
int k[];

extern int truly_external_array[];

void use() {
  y_internal = 50;
  truly_external = 100;
  truly_external_array[10] = 1;
}
