

#include "graph.hpp"

Graph::Graph() = default;

VertexID Graph::add_vertex() {
  VertexID id;
  // TODO: remove vertex properly
  if (!this->recently_removed.empty()) {
    auto it = this->recently_removed.begin();
    id = *it;
    this->recently_removed.erase(it);
  } else {
    id = adj.size();
    adj.emplace_back();
  }
  return id;
}

bool Graph::add_edge(VertexID v, VertexID w) {
  if (v >= this->adj.size() || w >= this->adj.size()) {
    return false;
  }

  if (this->recently_removed.contains(v) ||
      this->recently_removed.contains(w)) {
    return false;
  }

  adj[v].insert(w);
  adj[w].insert(v);
  return true;
}
