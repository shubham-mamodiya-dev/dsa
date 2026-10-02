/**
 * @file
 * @brief This file contains implementations of graphs and some algorithms
 * involving graphs.
 */

#pragma once

#include <cstdint>
#include <unordered_set>
#include <vector>

using VertexID = int64_t;

class Graph {
public:
  std::vector<std::unordered_set<VertexID>> adj;
  /**
   * @brief recently_removed keeps those vertices that are deleted explicitly
   * from the graph. These vertices are re-utilized afterwards.
   */
  std::unordered_set<VertexID> reusable;

public:
  Graph();

  /**
   * @brief it adds vertex then returns its id. This id will be used for
   * interacting with it.
   *
   * @return size_t or VertexID.
   */
  VertexID add_vertex();

  /**
   * @brief It adds edge between vertex v and vertex w only if v and w exists
   * in the graph.
   *
   * @return true if edge is added else false. false may mean two things either
   * v or w doesn't exist in the graph or one of them was recently removed.
   */
  bool add_edge(VertexID v, VertexID w);

  bool remove_vertex(VertexID id);
};
