#include <cstdint>
#include <cstdlib>
#include <iostream>
#include <stdlib.h>

template <typename T> struct node {
  T data;
  node<T> *next;
  node<T> *prev;
};

template <typename T> void insert(node<T> **head, node<T> *new_node);
template <typename T> void remove(node<T> **head, node<T> *target_to_remove);
template <typename T> node<T> *search(node<T> *head, T key);
template <typename T> void update(node<T> *head, T key);
template <typename T> node<T> *min(node<T> *head);
template <typename T> node<T> *max(node<T> *head);
template <typename T> node<T> *prev(node<T> *head, T key);
template <typename T> node<T> *next(node<T> *head, T key);

template <typename T> void print_list(node<T> *node);
template <typename T> void print_list(const char *prefix, node<T> *node);
template <typename T> void print_node(const char *prefix, node<T> *node);
