#!/bin/bash

# Build the Rust library
cd rust_lib
cargo build --release
cd ..

# Build the C++ project
mkdir -p build
cd build
cmake ..
make

cd ..
./build/main
