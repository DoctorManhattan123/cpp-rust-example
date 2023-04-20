cmake_minimum_required(VERSION 3.10)
project(my_cpp_project)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_BUILD_TYPE Release)

add_executable(my_cpp_project src/main.cpp)

# Link the Rust library
set(RUST_LIB_DIR "${CMAKE_CURRENT_SOURCE_DIR}/rust_lib/target/release")
find_library(RUST_LIB rust_lib HINTS "${RUST_LIB_DIR}")
target_link_libraries(my_cpp_project PRIVATE "${RUST_LIB}")
