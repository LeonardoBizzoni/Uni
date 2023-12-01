#ifndef _NODE_H
#define _NODE_H

typedef struct Node Node;

struct Node {
  int key;
  void *value;

  Node *left;
  Node *right;
};

extern Node *node_new(int, void*);
extern Node *node_insert(Node *, Node *);
extern Node *node_search(Node *, int);
extern void node_print(Node *);

extern void node_free(Node *);

#endif
