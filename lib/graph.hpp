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
};
