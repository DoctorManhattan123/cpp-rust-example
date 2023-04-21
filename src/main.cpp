#include <iostream>

// Declare the Rust function
extern "C" void rust_function(int x);
// Function that will be called from Rust
extern "C" void hello_from_cpp(double y) {
    std::cout << "Hello from C++! Received: " << y << std::endl;
}
int main() {
    std::cout << "Hello from C++!" << std::endl;
    rust_function(42);
    return 0;
}
