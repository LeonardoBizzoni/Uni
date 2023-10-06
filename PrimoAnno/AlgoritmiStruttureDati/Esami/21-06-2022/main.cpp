#include <cstddef>
#include <iostream>
#include <cstdint>
#include <ostream>
#include <stack>

#define X 1
#define Y 40 

int main() {
  bool inseritoX = false;
  bool inseritoY = false;

  std::stack<uint16_t> p;
  std::stack<uint16_t> app;

  for (size_t i=4; i<40; i+=2) {
    p.push(i);
  }

  while (!p.empty()) {
    if (!inseritoX && X>p.top()) {
      app.push(X);
      inseritoX = true;
    } else if (!inseritoY && Y>p.top()) {
      app.push(Y);
      inseritoY = true;
    } else {
      app.push(p.top());
      p.pop();
    }
  }

  if (!inseritoX && inseritoY) {
    app.push(X);
  } else if (!inseritoY && inseritoX) {
    app.push(Y);
  } else if (!inseritoX && !inseritoX) {
    if (X>=Y) {
      app.push(X);
      app.push(Y);
    } else {
      app.push(Y);
      app.push(X);
    }
  }

  while (!app.empty()) {
    p.push(app.top());
    app.pop();

    std::cout << p.top() << " ";
  }

  std::cout << std::endl;
}
