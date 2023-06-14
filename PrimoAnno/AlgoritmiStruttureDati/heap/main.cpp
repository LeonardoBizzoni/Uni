#include "lib.h"
#include <cstddef>
#include <cstdint>
#include <iostream>
#include <ostream>

int main() {
  Heap heap = Heap(10);

  std::cout << "Before sorting: " << heap.heap_string() << std::endl;
  auto sorted_heap = heap.heapsort();

  for (size_t i=0; i<10; i++) {
    std::cout << sorted_heap[i] << " ";
  }
  std::cout << std::endl;
}
