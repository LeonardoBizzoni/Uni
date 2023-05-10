#include "lib.h"

int main() {
  auto queue = new Queue<uint16_t>();
  std::cout << "Empty: " << (queue->is_empty() ? "yes" : "no") << std::endl;

  queue->enqueue((uint16_t)2);
  std::cout << "Enqueueed 2: Head is " << queue->head->value << std::endl;

  queue->enqueue((uint16_t)6);
  std::cout << "Enqueueed 6: Head is " << queue->head->value << std::endl;

  Node<uint16_t> first = queue->dequeue();
  std::cout << "Dequeueped " << first.value << std::endl;
  std::cout << "Head is " << queue->head->value << std::endl;

  std::cout << "Empty: " << (queue->is_empty() ? "yes" : "no") << std::endl;

  Node<uint16_t> second = queue->dequeue();
  std::cout << "Dequeueped " << second.value << std::endl;

  std::cout << "Empty: " << (queue->is_empty() ? "yes" : "no") << std::endl;
}
