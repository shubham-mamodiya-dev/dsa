/**
 * @file
 * @brief This file contains implementations of graphs and some algorithms
 * involving graphs.
 */

#pragma once

#include <cstddef>
#include <unordered_set>
#include <vector>

using VertexID = size_t;

class Graph {
  std::vector<std::unordered_set<VertexID>> adj;

public:
  Graph();

  VertexID add_vertex();

  /**
   * @brief It adds edge between vertex v and vertex w only if v and w exists
   * in the graph.
   *
   * @return true if edge is added else false.
   */
  bool add_edge(VertexID v, VertexID w);
};
