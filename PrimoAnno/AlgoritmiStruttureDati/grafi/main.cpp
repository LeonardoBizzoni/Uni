#include "lib.h"
#include <cstddef>
#include <cstdint>

int main() {
  Node<uint16_t> vertices[] = { Node<uint16_t>(1),
			       Node<uint16_t>(10),
			       Node<uint16_t>(20),
			       Node<uint16_t>(30),
			       Node<uint16_t>(40),
			       Node<uint16_t>(50),
			       Node<uint16_t>(60),
			       Node<uint16_t>(70),
			       Node<uint16_t>(80) };
  size_t size = sizeof(vertices)/sizeof(Node<uint16_t>);

  vertices[0].add_adj(&vertices[1]);
  vertices[0].add_adj(&vertices[3]);
  vertices[0].add_adj(&vertices[7]);
  vertices[0].add_adj(&vertices[2]);

  vertices[5].add_adj(&vertices[4]);
  vertices[5].add_adj(&vertices[8]);
  vertices[5].add_adj(&vertices[3]);

  vertices[7].add_adj(&vertices[8]);
  vertices[8].add_adj(&vertices[8]);
  vertices[8].add_adj(&vertices[0]);

  Graph<uint16_t> graph = Graph<uint16_t>(vertices, size);
  // graph.bfs(&vertices[7]);
  graph.dfs();

  for (auto i=0; i<size; i++) {
    vertices[i].print();
  }
}
