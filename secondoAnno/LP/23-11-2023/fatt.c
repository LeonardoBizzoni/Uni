#include "factorial.h"

int main(int argc, char **argv) {
  if (argc < 2) {
    exit(EXIT_FAILURE);
  }

  for (int i = 1; i < argc; i++) {
    if (*argv[i] >= '0' && *argv[i] <= '9') {
      int n = atoi(argv[i]);
      printf("%d! = %d\n", n, factor(n));
    }
  }
}
