#include "lib.h"

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
