

#include "graph.hpp"

Graph::Graph() = default;

VertexID Graph::add_vertex() {
  VertexID id;
  if (!this->reusable.empty()) {
    auto it = this->reusable.begin();
    id = *it;
    this->remove_vertex(id);
  } else {
    id = adj.size();
    adj.emplace_back();
  }
  return id;
}

bool Graph::add_edge(VertexID v, VertexID w) {
  if (v < 0 || w < 0) {
    return false;
  }
  if (v >= this->adj.size() || w >= this->adj.size()) {
    return false;
  }

  if (this->reusable.contains(v) || this->reusable.contains(w)) {
    return false;
  }

  adj[v].insert(w);
  adj[w].insert(v);
  return true;
}

bool Graph::remove_vertex(VertexID id) {
  if (id < 0) {
    return false;
  }
  // Can't remove it because it is not in the graph
  if (id >= this->adj.size()) {
    return false;
  }

  // Can't remove it because it is already removed
  if (this->reusable.contains(id)) {
    return false;
  }

  // make id reusable.
  this->reusable.insert(id);

  // Remove all the incoming connections.
  for (const auto v : this->adj[id]) {
    this->adj[v].erase(id);
  }

  // Remove all the outgoing connections.
  this->adj[id].clear();

  return true;
}
