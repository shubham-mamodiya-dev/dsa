cmake_minimum_required(VERSION 3.25)


project("dsa_implementaions" VERSION 1.0.0 DESCRIPTION "Implementations of Data Structure and Algorithms." LANGUAGES CXX)


set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)


add_subdirectory("lib")
add_subdirectory("src")


include(FetchContent)

FetchContent_Declare(
  fmt
  GIT_REPOSITORY https://github.com/fmtlib/fmt.git
  GIT_TAG 12.2.0
)
FetchContent_MakeAvailable(fmt)
