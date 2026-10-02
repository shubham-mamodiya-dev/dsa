

#include "graph.hpp"
#include <cstddef>

Graph::Graph() = default;

VertexID Graph::add_vertex() {
  VertexID id;
  if (!this->reusable.empty()) {
    auto it = this->reusable.begin();
    id = *it;
    this->reusable.erase(id);
  } else {
    id = adj.size();
    adj.emplace_back();
  }
  return id;
}

bool Graph::add_edge(VertexID v, VertexID w) {
  if (!this->is_valid_vertex(v) || !this->is_valid_vertex(w)) {
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
  if (!this->is_valid_vertex(id)) {
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

size_t Graph::vertex_count() const {
  return this->adj.size() - this->reusable.size();
}

size_t Graph::edge_count() const {

  // NOTE: vertices that are removed have 0 elements in there adj.
  size_t total_edges = 0;
  for (const auto &x : this->adj) {
    total_edges += x.size();
  }

  // In a graph edges are bidirectional and counted twice.
  total_edges /= 2;

  return total_edges;
}

bool Graph::is_connected(VertexID v, VertexID w) const {
  if (!this->is_valid_vertex(v) || !this->is_valid_vertex(w)) {
    return false;
  }

  // Edges in graphs are bidirectional. But, We are checking only for one
  // direction
  return this->adj[v].contains(w);
}

bool Graph::is_reusable(VertexID v) const { return this->reusable.contains(v); }

bool Graph::is_valid_vertex(VertexID v) const {
  return v >= 0 && v < static_cast<VertexID>(adj.size()) &&
         !reusable.contains(v);
}
