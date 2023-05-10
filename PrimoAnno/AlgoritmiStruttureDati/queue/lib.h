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

template <typename T> class Queue {
private:
  Node<T> *top;
public:
  Node<T> *head;

  Queue<T>(): top(nullptr), head(nullptr) {}

  void enqueue(T value) {
    Node<T> *next_node = new Node<T>(value);

    if (top == nullptr) {
      top = next_node;
      head = next_node;
    } else {
      next_node->prev = top;
      top->next = next_node;
      top = next_node;
    }
  }

  Node<T> dequeue() {
    Node<T> res = *head;

    if (head != nullptr) {
      head = head->next;
    }

    return res;
  }

  bool is_empty() {
    return head == nullptr;
  }
};
