#include "main.h"

int main() {
  node<uint16_t> *head;
  auto one = (node<uint16_t> *)malloc(sizeof(node<uint16_t>));
  auto two = (node<uint16_t> *)malloc(sizeof(node<uint16_t>));
  auto three = (node<uint16_t> *)malloc(sizeof(node<uint16_t>));

  one->data = 10;
  two->data = 20;
  three->data = 30;

  one->next = two;
  two->next = three;

  two->prev = one;
  three->prev = two;

  head = one;

  print_list("Created: ", head);

  update(two, (uint16_t)34);
  print_list("Updated: ",head);

  auto new_node = (node<uint16_t> *)malloc(sizeof(node<uint16_t>));
  new_node->data = 56;
  insert(&head, new_node);
  print_list("Inserted: ", head);

  auto new_node2 = (node<uint16_t> *)malloc(sizeof(node<uint16_t>));
  new_node2->data = 0;
  insert(&head, new_node2);
  print_list("Inserted: ", head);

  auto res_search = search(head, (uint16_t)10);
  print_node("Search: ", res_search);

  auto min_ptr = min(head);
  print_node("Min: ", min_ptr);

  auto max_ptr = max(head);
  print_node("Max: ", max_ptr);

  auto next_ptr = next(head, (uint16_t)30);
  print_node("Next: ", next_ptr);

  auto prev_ptr = prev(head, (uint16_t)10);
  print_node("Prev: ", prev_ptr);

  remove(&head, new_node);
  print_list("Removed: ", head);

  remove(&head, new_node2);
  print_list("Removed: ", head);

  free(head);
  free(two);
  free(three);
}

template <typename T> void print_list(node<T> *node) {
  print_list("", node);
}

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
    std::cout << "-> "  << node->next->data;
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

template <typename T> void update(node<T> *node, T key) {
  node->data = key;
}

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
