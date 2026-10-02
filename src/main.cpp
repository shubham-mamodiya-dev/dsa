#include "fmt/base.h"
#include <fmt/format.h>
#include <fmt/ranges.h>
#include <graph.hpp>
#include <iostream>
#include <vector>
int main() {
  std::cout
      << "==================== DSA Implementations ====================\n";

  Graph g{};
  std::vector<VertexID> vertices{};

  for (int i = 0; i < 20; ++i) {
    vertices.push_back(g.add_vertex());
  }

  g.add_edge(vertices[0], vertices[1]);
  g.add_edge(vertices[0], vertices[2]);
  g.add_edge(vertices[0], vertices[5]);

  g.add_edge(vertices[1], vertices[3]);
  g.add_edge(vertices[1], vertices[4]);

  g.add_edge(vertices[2], vertices[4]);
  g.add_edge(vertices[2], vertices[6]);
  g.add_edge(vertices[2], vertices[7]);

  g.add_edge(vertices[3], vertices[8]);

  g.add_edge(vertices[4], vertices[8]);
  g.add_edge(vertices[4], vertices[9]);

  g.add_edge(vertices[5], vertices[10]);
  g.add_edge(vertices[5], vertices[11]);

  g.add_edge(vertices[6], vertices[12]);
  g.add_edge(vertices[7], vertices[12]);
  g.add_edge(vertices[7], vertices[13]);

  g.add_edge(vertices[8], vertices[14]);
  g.add_edge(vertices[9], vertices[15]);

  g.add_edge(vertices[10], vertices[16]);
  g.add_edge(vertices[11], vertices[17]);

  g.add_edge(vertices[12], vertices[18]);
  g.add_edge(vertices[13], vertices[19]);

  fmt::println("adj: {}", g.adj);
  fmt::println("Recently Removed: ", g.recently_removed);
  return 0;
}
