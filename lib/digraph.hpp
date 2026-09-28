/**
 * @file
 * @brief This file contains implementations of digraph and some algorithms
 * involving digraph.
 */

#pragma once

#include <cstddef>
#include <unordered_set>
#include <vector>

using VertexID = size_t;

class Digraph {
  std::vector<std::unordered_set<VertexID>> adj;

public:
  Digraph();
};
