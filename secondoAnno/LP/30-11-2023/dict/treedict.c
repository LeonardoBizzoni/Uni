#include "treedict.h"
#include "node.h"

#include <stdio.h>
#include <stdlib.h>

Treedict *dict_new(char *name) {
  Treedict *dict = malloc(sizeof(Treedict));
  if (!dict) {
    fprintf(stderr, "treedict: dict_new: cannot allocate memory.\n");
    exit(EXIT_FAILURE);
  }

  dict->name = name;

  return dict;
}

void dict_insert(Treedict *dict, int key, char *value) {
  dict->root = node_insert(dict->root, node_new(key, value));
}

Node *dict_search(Treedict *dict, int key) {
  return node_search(dict->root, key);
}

void dict_print(Treedict *dict) {
  printf("TreeDict { name: \"%s\", root: ", dict->name);
  node_print(dict->root);
  printf(" }\n");
}

void dict_free(Treedict *dict) {
  free(dict->name);
  node_free(dict->root);

  free(dict);
}
