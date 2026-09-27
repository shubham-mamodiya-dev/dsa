add_library("dsa" STATIC
  graph.cpp
  graph.hpp
)

target_include_directories("dsa" INTERFACE ${CMAKE_CURRENT_SOURCE_DIR})
