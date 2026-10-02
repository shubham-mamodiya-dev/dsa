

#include "graph.hpp"
#include <cstddef>

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

  // NOTE: reusable vertices are the vertices that were deleted before.
  if (this->is_reusable(v) || this->is_reusable(w)) {
    return false;
  }

  // A vertex is connected to itself and We do not keep entries for that.
  if (v == w) {
    return true;
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
  if (this->is_reusable(id)) {
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

size_t Graph::vertex_count() {
  return this->adj.size() - this->reusable.size();
}

size_t Graph::edge_count() {

  // NOTE: vertices that are removed have 0 elements in there adj.
  size_t total_edges = 0;
  for (const auto &x : this->adj) {
    total_edges += x.size();
  }

  // In a graph edges are bidirectional and counted twice.
  total_edges /= 2;

  return total_edges;
}

bool Graph::is_connected(VertexID v, VertexID w) {
  if (v < 0 || w < 0) {
    return false;
  }

  if (v >= this->adj.size() || w >= this->adj.size()) {
    return false;
  }

  // NOTE: reusable vertices are the vertices that were deleted before.
  if (this->is_reusable(v) || this->is_reusable(w)) {
    return false;
  }

  // Edges in graphs are bidirectional. But, We are checking only for one
  // direction
  return this->adj[v].contains(w);
}

bool Graph::is_reusable(VertexID v) { return this->reusable.contains(v); }
