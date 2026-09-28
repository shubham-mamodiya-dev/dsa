

#include "graph.hpp"

Graph::Graph() : adj{} {}

VertexID Graph::add_vertex() {
  VertexID id{adj.size()};
  adj.emplace_back();
  return id;
}

bool Graph::add_edge(VertexID v, VertexID w) {
  if (v >= adj.size() || w >= adj.size()) {
    return false;
  }

  adj[v].insert(w);
  adj[w].insert(v);
  return true;
}
