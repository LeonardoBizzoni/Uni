#include <cstddef>
#include <cstdint>
#include <iostream>
#include <list>
#include <queue>
#include <iterator>
#include <type_traits>

template <typename T> class Node {
public:
  int16_t distanza = -1;
  int16_t tempo_scoperto = -1;
  int16_t tempo_completato = -1;
  Node<T> *prev = nullptr;
  std::string color = "white";

public:
  T key;
  std::list<Node<T> *> adj;

 public:
  Node<T>(T key) : key(key) {}

  void add_adj(Node<T> *other) {
    adj.push_back(other);
  }

  void print() {
    std::cout << "key: " << key << "\tAdj: { ";

    for (Node<T> *node : adj) {
      std::cout << node << " ";
    }
    std::cout << "}\n\tcolor: " << color << std::endl;

    if (distanza != -1) {
      std::cout << "\tdistanza: " << distanza << std::endl;
    }
    if (tempo_scoperto != -1 && tempo_completato != -1) {
      std::cout << "\tscoperto: " << tempo_scoperto << std::endl;
      std::cout << "\tcompletato: " << tempo_completato << std::endl;
    }
  }
};

template<typename T> class Graph {
private:
  Node<T> *vertici;
  size_t size;
  int16_t time = 0;

public:
  Graph<T>(Node<T> vertici[], size_t size) : vertici(vertici), size(size) { }

  void bfs(Node<T> *node) {
    node->distanza = 0;
    std::queue<Node<T> *> q;
    q.push(node);

    while (!q.empty()) {
      Node<T> *u = q.front();
      q.pop();

      for (Node<T> *v : u->adj) {
	if (v->color == "white") {
	  v->color = "grey";
	  v->distanza = u->distanza + 1;
	  v->prev = u;
	  q.push(v);
	}
      }

      u->color = "black";
    }
  }

  void dfs() {
    for (size_t i = 0; i < size; i++) {
      auto u = vertici+i;

      u->tempo_scoperto = 0;
      u->tempo_completato = 0;
    }

    for (size_t i = 0; i < size; i++) {
      auto u = vertici+i;

      if (u->color == "white") {
	dfs_visit(u);
      }
    }
  }

private:
  void dfs_visit(Node<T> *pNode) {
    pNode->color = "grey";
    pNode->tempo_scoperto = time;
    time++;

    for (Node<T> *v : pNode->adj) {
      if (v->color == "white") {
	v->prev = pNode;
	dfs_visit(v);
      }
    }

    pNode->color = "black";
    pNode->tempo_completato = time;
    time++;
  }
};
