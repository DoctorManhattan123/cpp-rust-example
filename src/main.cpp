#include <iostream>

// Declare the Rust function
extern "C" void rust_function();

int main() {
    std::cout << "Hello from C++!" << std::endl;
    rust_function();
    return 0;
}
