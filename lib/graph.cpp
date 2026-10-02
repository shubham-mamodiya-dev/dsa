

#include "graph.hpp"
#include <cstddef>

Graph::Graph() = default;

VertexID Graph::add_vertex() {
  // First reusing vertices.
  if (!this->reusable.empty()) {
    const VertexID id = *this->reusable.begin();
    this->reusable.erase(id);
    return id;
  }

  const VertexID id = adj.size();
  adj.emplace_back();
  return id;
}

bool Graph::add_edge(const VertexID v, const VertexID w) {
  if (!this->is_valid_vertex(v) || !this->is_valid_vertex(w)) {
    return false;
  }

  // A vertex is connected to itself and We do not keep entries for that.
  if (v == w) {
    return true;
  }

  this->adj[v].insert(w);
  this->adj[w].insert(v);
  this->_edge_count += 1;
  return true;
}

bool Graph::remove_vertex(const VertexID id) {
  if (!this->is_valid_vertex(id)) {
    return false;
  }

  // make id reusable.
  this->reusable.insert(id);

  // Remove all the incoming connections.
  for (const auto v : this->adj[id]) {
    this->adj[v].erase(id);
  }

  this->_edge_count -= static_cast<int64_t>(adj[id].size());

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

bool Graph::is_connected(const VertexID v, const VertexID w) const {
  if (!this->is_valid_vertex(v) || !this->is_valid_vertex(w)) {
    return false;
  }

  // Edges in graphs are bidirectional. But, We are checking only for one
  // direction
  return this->adj[v].contains(w);
}

bool Graph::is_reusable(const VertexID v) const {
  return this->reusable.contains(v);
}

bool Graph::is_valid_vertex(const VertexID v) const {
  return v >= 0 && v < static_cast<VertexID>(adj.size()) &&
         !reusable.contains(v);
}

bool Graph::remove_edge(const VertexID v, const VertexID w) {
  if (!this->is_valid_vertex(v) || !this->is_valid_vertex(w)) {
    return false;
  }

  this->adj[v].erase(w);
  this->adj[w].erase(v);
  this->_edge_count -= 1;
  return true;
}
