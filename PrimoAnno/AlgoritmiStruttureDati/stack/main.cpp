#include "lib.h"

int main() {
  auto stack = new Stack<uint16_t>();
  std::cout << "Empty: " << (stack->is_empty() ? "yes" : "no") << std::endl;

  stack->push((uint16_t)2);
  stack->push((uint16_t)6);

  std::cout << stack->top->value << std::endl;

  Node<uint16_t> popped_value = stack->pop();

  std::cout << popped_value.value << std::endl;
  std::cout << stack->top->value << std::endl;
  std::cout << "Empty: " << (stack->is_empty() ? "yes" : "no") << std::endl;

  stack->pop();
  std::cout << "Empty: " << (stack->is_empty() ? "yes" : "no") << std::endl;
}
