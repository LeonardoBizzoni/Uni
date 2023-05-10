#pragma once
#include <cstdint>
#include <cstdlib>
#include <iostream>
#include <stdlib.h>

template <typename T> class Node {
public:
  T value;
  Node<T> *next;
  Node<T> *prev;

  Node<T>(T value) : value(value), next(nullptr), prev(nullptr) {}
};

template <typename T> class Stack {
public:
  Node<T> *top;

  Stack<T>(): top(nullptr) {}

  void push(T value) {
    Node<T> *next_node = new Node<T>(value);

    if (top == nullptr) {
      top = next_node;
    } else {
      next_node->prev = top;
      top->next = next_node;
      top = next_node;
    }
  }

  Node<T> pop() {
    Node<T> res = *top;
    top = top->prev;

    if (top != nullptr) {
      top->next = nullptr;
    }

    return res;
  }

  bool is_empty() {
    return top == nullptr;
  }
};
