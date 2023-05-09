#include "lib.h"
#include <iostream>

template <typename T> void print_list(node<T> *node) { print_list("", node); }

template <typename T> void print_list(const char *prefix, node<T> *node) {
  if (node == nullptr) {
    std::cout << std::endl;
    return;
  }

  std::cout << prefix << node->data << "\t";
  print_list(node->next);
}

template <typename T> void print_node(const char *prefix, node<T> *node) {
  std::cout << prefix;
  if (node == nullptr) {
    std::cout << "NULL" << std::endl;
    return;
  }

  std::cout << node << ": ";
  if (node->prev != nullptr) {
    std::cout << node->prev->data << " <-";
  }
  std::cout << node->data;
  if (node->next != nullptr) {
    std::cout << "-> " << node->next->data;
  }
  std::cout << std::endl;
}

template <typename T> void insert(node<T> **head, node<T> *new_node) {
  if (head != nullptr) {
    (*head)->prev = new_node;
  }

  new_node->next = *head;
  *head = new_node;
}

template <typename T> void remove(node<T> **head, node<T> *target_to_remove) {
  if (target_to_remove->prev != nullptr) {
    target_to_remove->prev->next = target_to_remove->next;
  } else {
    *head = target_to_remove->next;
  }

  if (target_to_remove->next != nullptr) {
    target_to_remove->next->prev = target_to_remove->prev;
  } else {
    target_to_remove->prev->next = nullptr;
  }

  free(target_to_remove);
}

template <typename T> node<T> *search(node<T> *head, T key) {
  node<T> *current = head;
  while (current != nullptr && current->data != key) {
    current = current->next;
  }

  return current;
}

template <typename T> void update(node<T> *node, T key) { node->data = key; }

template <typename T> node<T> *min(node<T> *head) {
  auto current = head;
  auto min = current;

  while (current != nullptr) {
    if (min->data > current->data) {
      min = current;
    }

    current = current->next;
  }

  return min;
}

template <typename T> node<T> *max(node<T> *head) {
  auto current = head;
  auto max = current;

  while (current != nullptr) {
    if (max->data < current->data) {
      max = current;
    }

    current = current->next;
  }

  return max;
}

template <typename T> node<T> *next(node<T> *head, T key) {
  auto current = head;
  node<T> *next = nullptr;

  while (next == nullptr && current != nullptr) {
    if (current->data > key) {
      next = current;
    }
    current = current->next;
  }

  while (current != nullptr) {
    if (next->data > current->data && current->data > key) {
      next = current;
    }

    current = current->next;
  }

  return next;
}

template <typename T> node<T> *prev(node<T> *head, T key) {
  auto current = head;
  node<T> *prev = nullptr;

  while (prev == nullptr && current != nullptr) {
    if (current->data < key) {
      prev = current;
    }
    current = current->next;
  }

  while (current != nullptr) {
    if (prev->data < current->data && current->data < key) {
      prev = current;
    }

    current = current->next;
  }

  return prev;
}
