#include "node.h"

#include <stdio.h>
#include <stdlib.h>

Node *node_new(int key, void *value) {
  Node *node = malloc(sizeof(Node));
  if (!node) {
    fprintf(stderr, "node: node_new: cannot allocate memory.\n");
    exit(EXIT_FAILURE);
  }

  node->key = key;
  node->value = value;

  return node;
}

Node *node_insert(Node *root, Node *node) {
  if (root == NULL) {
    return node;
  } else if (node->key > root->key) {
    root->right = node_insert(root->right, node);
  } else if (node->key < root->key) {
    root->left = node_insert(root->left, node);
  } else {
    root->value = node->value;
  }

  return root;
}

Node *node_search(Node *node, int key) {
  if (node == NULL || node->key == key) {
    return node;
  }

  if (key > node->key) {
    return node_search(node->right, key);
  } else {
    return node_search(node->left, key);
  }
}

void node_print(Node *node) {
  if (node == NULL) {
    printf("Node { nil }");
    return;
  }

  printf("Node { key: %d, value: \"%s\", left: ", node->key, node->value);
  node_print(node->left);
  printf(", right: ");
  node_print(node->right);
  printf(" }");
}

void node_free(Node *node) {
  if (node == NULL) {
    return;
  }

  if (node->left != NULL) {
    node_free(node->left);
  }

  if (node->right != NULL) {
    node_free(node->right);
  }

  free(node);
}
