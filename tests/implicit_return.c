// ret 0
int main(void) {}

// ret void
void func(void) {}

void branch(void) {
  int i;
  if (i) {
    i = 10;
  } else {
    i++;
  }
}

// unreachable
int func2(void) {}