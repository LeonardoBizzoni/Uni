#ifndef _TREEDICT_H
#define _TREEDICT_H

#include "node.h"

typedef struct {
  char *name;
  Node *root;
} Treedict;

extern Treedict *dict_new(char *);

extern void dict_insert(Treedict *, int, char *);
extern Node *dict_search(Treedict *, int);
extern void dict_remove(Treedict *, int);
extern void dict_print(Treedict *);

extern void dict_free(Treedict *);

#endif
