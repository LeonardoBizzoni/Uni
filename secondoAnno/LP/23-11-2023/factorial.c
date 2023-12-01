#include "factorial.h"

int factor(int n) {
  if (n < 0) {
    fprintf(stderr, "Numero negativo\n");
    exit(EXIT_FAILURE);
  } else if (n == 0) {
    return 1;
  }

  return n * factor(n - 1);
}
