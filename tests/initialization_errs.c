// Error: String too long for the array
char s2[2] = "abc";

// Error: Designator index outside array bounds
int err2[5] = {[10] = 1};

// Error: Negative index
int err3[5] = {[-1] = 1};

int err4[3] = {{[0] = 0}, {{{{{{{{{{1}}}}}}}}}}, {{{{{{{}}}}}}}};

const unsigned long long L = 100;
// non-standard: L got folded to a constant.
int arr[L];

void _() {
  // clang treat this as VLA still(because VLA can be local decl, but not
  // global), but my implementation folds it to a constant array.
  const int L2 = 100;
  int arr2[L2] = {};
}

constexpr auto L3 = 5;
unsigned vla_fold[L3];