#include <cstddef>
#include <cstdint>
#include <cstdlib>
#include <ctime>
#include <iostream>
#include <ostream>
#include <string>
#include <vector>

#define LENGHT 100

class Heap {
private:
  uint16_t m_arr[LENGHT];
  const uint16_t c_heapsize;

public:
  uint16_t m_heapsize;

public:
  Heap(uint16_t p_heapsize)
      : m_heapsize(p_heapsize > LENGHT ? LENGHT : p_heapsize),
        c_heapsize(p_heapsize > LENGHT ? LENGHT : p_heapsize) {
    std::srand(std::time(nullptr));

    for (size_t i = 0; i < m_heapsize; i++) {
      m_arr[i] = std::rand() % 100;
    }

    build_heap();
  }

  void build_heap() {
    for (size_t i = (m_heapsize / 2); i > 0; i--) {
      heapify(i, m_arr, m_heapsize);
    }

    // size_t non assume valori negativi
    heapify(0, m_arr, m_heapsize);
  }

  std::string heap_string() {
    std::string res = "";

    for (size_t i = 0; i < m_heapsize; i++) {
      res += std::to_string(i + 1) + ":" + std::to_string(m_arr[i]) + " ";
    }

    return res;
  }

  uint16_t *heapsort() {
    uint16_t heapsize = m_heapsize-1;
    uint16_t *arr = m_arr;

    //build_heap();
    for (size_t i=0; i<m_heapsize-1; i++) {
      auto tmp = arr[0];
      arr[0] = arr[heapsize];
      arr[heapsize] = tmp;

      heapify(0, arr, heapsize);
      heapsize--;
    }

    return arr;
  }

private:
  void heapify(size_t i, uint16_t *p_arr, uint16_t p_heapsize) {
    size_t max = i;
    size_t left = 2 * i;
    size_t right = (2 * i) + 1;

    if (left < p_heapsize && p_arr[left] > p_arr[max]) {
      max = left;
    }
    if (right < p_heapsize && p_arr[right] > p_arr[max]) {
      max = right;
    }

    if (p_arr[max] > p_arr[i]) {
      uint16_t tmp = p_arr[i];
      p_arr[i] = p_arr[max];
      p_arr[max] = tmp;

      heapify(max, p_arr, p_heapsize);
    }
  }
};
