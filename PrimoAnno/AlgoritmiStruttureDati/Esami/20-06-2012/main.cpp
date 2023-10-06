#include <cstddef>
#include <stack>
#include <cstdint>
#include <ctime>
#include <iostream>

#define N 10

int main() {
  // INIT
  srand(time(0));
  std::stack<uint8_t> p1;
  std::stack<uint8_t> p2;
  std::stack<uint8_t> p3;

  std::cout << "P1: ";
  for (size_t i = 0; i < N; i++) {
    p1.push('a' + (rand() % ('z' - 'a')));
    std::cout << p1.top() << " ";
  }
  std::cout << std::endl << "P2: ";
  for (size_t i = 0; i < N; i++) {
    p2.push('a' + (rand() % ('z' - 'a')));
    std::cout << p2.top() << " ";
  }
  std::cout << std::endl;

  uint8_t car = 'a' + (rand() % ('z' - 'a'));
  std::cout << "CAR: " << car << std::endl << "P3: ";
  // -------------------------------------------------

  size_t i = 0;
  uint8_t att;
  while (!p1.empty() || !p2.empty()) {
    if (i%2) {
      att = p1.top();
      p1.pop();
    } else {
      att = p2.top();
      p2.pop();
    }

    if (att != car) {
      p3.push(att);
      std::cout << p3.top() << " ";
    }

    i++;
  }
  std::cout << std::endl;
}
