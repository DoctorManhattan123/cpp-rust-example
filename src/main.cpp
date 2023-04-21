#include <iostream>

// Declare the Rust function
extern "C" void rust_function(int x);

// Main function to start the project.
int main() {
    std::cout << "Hello from C++!" << std::endl;
    rust_function(42);
    return 0;
}
