#include "lib.h"
#include <cstdint>
#include <iostream>

int main() {
  BST<uint16_t> bst = BST<uint16_t>(20);
  bst.insert(23, bst.root);
  bst.insert(15, bst.root);
  bst.insert(9, bst.root);
  bst.insert(30, bst.root);
  bst.insert(17, bst.root);

  std::cout << "Ricerca un valore: ";
  std::cout << bst.search(50, bst.root) << std::endl;

  std::cout << "Min: " << bst.min(bst.root) << std::endl;
  std::cout << "Max: " << bst.max(bst.root) << std::endl;

  std::cout << "Visita anticipata: ";
  bst.visita_anticipata(bst.root);

  std::cout << "\nVisita simmetrica: ";
  bst.visita_simmetrica(bst.root);

  std::cout << "\nVisita posticipata: ";
  bst.visita_posticipata(bst.root);
}
