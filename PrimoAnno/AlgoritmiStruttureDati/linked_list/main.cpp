#include "main.h"

int main() {
  node<uint16_t> *head;
  auto *one = (node<uint16_t> *)malloc(sizeof(node<uint16_t>));
  auto *two = (node<uint16_t> *)malloc(sizeof(node<uint16_t>));
  auto *three = (node<uint16_t> *)malloc(sizeof(node<uint16_t>));

  one->data = 10;
  two->data = 20;
  three->data = 30;

  one->next = two;
  two->next = three;

  two->prev = one;
  three->prev = two;

  head = one;

  print_list(head);

  auto *new_node = (node<uint16_t> *)malloc(sizeof(node<uint16_t>));
  new_node->data = 56;
  insert(&head, new_node);
  print_list(head);

  auto *new_node2 = (node<uint16_t> *)malloc(sizeof(node<uint16_t>));
  new_node2->data = 0;
  insert(&head, new_node2);
  print_list(head);

  auto *res_search = search(head, (uint16_t)20);
  print_node(res_search);

  remove(&head, new_node);
  print_list(head);

  remove(&head, new_node2);
  print_list(head);

  free(head);
  free(two);
  free(three);
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

template <typename T> void print_list(node<T> *node) {
  if (node == nullptr) {
    std::cout << std::endl;
    return;
  }

  std::cout << node->data << "\t";
  print_list(node->next);
}

template <typename T> void print_node(node<T> *node) {
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
    std::cout << "-> "  << node->next->data;
  }
  std::cout << std::endl;
}
