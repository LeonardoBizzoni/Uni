#pragma once
#include <cstdint>
#include <cstdlib>
#include <iostream>
#include <stdlib.h>

template <typename T> class TreeNode {
private:
  TreeNode<T> *parent;

public:
  T key;
  TreeNode<T> *left;
  TreeNode<T> *right;

  TreeNode<T>(T key) : key(key), left(nullptr), right(nullptr), parent(nullptr) {}
};

template<typename T> class BST {
public:
  TreeNode<T> *root;

public:
  BST<T>(T key) : root(new TreeNode<T>(key)) {}

  TreeNode<T> *insert(T new_node, TreeNode<T> *branch) {
    if (branch == nullptr) {
      return new TreeNode<T>(new_node);
    }

    if (new_node > branch->key) {
      branch->right = insert(new_node, branch->right);
    } else {
      branch->left = insert(new_node, branch->left);
    }

    return branch;
  }

  TreeNode<T> *search(T value, TreeNode<T> *node) {
    if (node == nullptr || node->key == value) {
      return node;
    }

    if (value > node->key) {
      return search(value, node->right);
    }
    return search(value, node->left);
  }

  TreeNode<T> *min(TreeNode<T> *node) {
    if (node == nullptr || node->left == nullptr) {
      return node;
    }

    return min(node->left);
  }

  TreeNode<T> *max(TreeNode<T> *node) {
    if (node == nullptr || node->right == nullptr) {
      return node;
    }

    return max(node->right);
  }

  void visita_anticipata(TreeNode<T> *node) {
    if (node == nullptr) {
      return;
    }

    std::cout << node->key << " ";
    visita_anticipata(node->left);
    visita_anticipata(node->right);
  }

  void visita_simmetrica(TreeNode<T> *node) {
    if (node == nullptr) {
      return;
    }

    visita_simmetrica(node->left);
    std::cout << node->key << " ";
    visita_simmetrica(node->right);
  }

  void visita_posticipata(TreeNode<T> *node) {
    if (node == nullptr) {
      return;
    }

    visita_posticipata(node->left);
    visita_posticipata(node->right);
    std::cout << node->key << " ";
  }
};
