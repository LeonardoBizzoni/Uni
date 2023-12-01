#include "dict/treedict.h"
#include <stdio.h>
#include <stdlib.h>

int main(void) {
  char *name;

  printf("Inserisci nome treedict: ");
  scanf("%ms", &name);

  Treedict *dict = dict_new(name);

  dict_insert(dict, 2000, "Gladiator");
  dict_insert(dict, 2016, "La La Land");
  dict_insert(dict, 2023, "Barbie");
  dict_insert(dict, 2022, "The Power of the Dog");
  dict_insert(dict, 1980, "Blues Brothers");
  dict_insert(dict, 1974, "2001: A Space Odissey");
  dict_insert(dict, 1977, "Star Wars Episode IV");
  dict_print(dict);

  dict_free(dict);
}
