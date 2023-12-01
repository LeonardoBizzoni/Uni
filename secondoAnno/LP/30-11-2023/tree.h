#ifndef _NODE_H
#define _NODE_H

typedef struct Node Node;

struct Node {
  int key;
  char *value;

  Node *left;
  Node *right;
};

#endif
