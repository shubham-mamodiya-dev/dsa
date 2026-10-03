

#include "digraph.hpp"
#include <cstddef>

Digraph::Digraph() = default;

VertexID Digraph::add_vertex() {
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

bool Digraph::add_edge(const VertexID v, const VertexID w) {
  if (!this->is_valid_vertex(v) || !this->is_valid_vertex(w)) {
    return false;
  }

  // A vertex is connected to itself and We do not keep entries for that.
  if (v == w) {
    return true;
  }

  this->adj[v].insert(w);
  this->_edge_count += 1;
  return true;
}

bool Digraph::remove_vertex(const VertexID id) {
  if (!this->is_valid_vertex(id)) {
    return false;
  }

  // make id reusable.
  this->reusable.insert(id);

  // Remove all the incoming connections.
  for (const auto v : this->adj[id]) {
    this->adj[v].erase(id);

    // Edges in digraphs are not bidirectional. If you can travel
    // bidirectionally then that means there are two different edges.
    this->_edge_count -= 1;
  }

  // Edges in digraphs are not bidirectional. If you can travel
  // bidirectionally then that means there are two different edges.
  this->_edge_count -= static_cast<int64_t>(adj[id].size());

  // Remove all the outgoing connections.
  this->adj[id].clear();

  return true;
}

size_t Digraph::vertex_count() const {
  return this->adj.size() - this->reusable.size();
}

size_t Digraph::edge_count() const { return this->_edge_count; }

bool Digraph::is_connected(const VertexID v, const VertexID w) const {
  if (!this->is_valid_vertex(v) || !this->is_valid_vertex(w)) {
    return false;
  }

  // means v is connected to w.
  return this->adj[v].contains(w);
}

bool Digraph::is_reusable(const VertexID v) const {
  return this->reusable.contains(v);
}

bool Digraph::is_valid_vertex(const VertexID v) const {
  return v >= 0 && v < static_cast<VertexID>(adj.size()) &&
         !reusable.contains(v);
}

bool Digraph::remove_edge(const VertexID v, const VertexID w) {
  if (!this->is_valid_vertex(v) || !this->is_valid_vertex(w)) {
    return false;
  }

  this->adj[v].erase(w);
  this->_edge_count -= 1;
  return true;
}

std::vector<VertexID> Digraph::adjacent_vertices(const VertexID v) {
  if (this->is_valid_vertex(v)) {
    return {this->adj[v].begin(), this->adj[v].end()};
  }
  return {};
}
