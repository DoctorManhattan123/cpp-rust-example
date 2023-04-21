#include <iostream>

// Declare the Rust function
extern "C" void rust_function();

// Function that will be called from Rust
extern "C" void hello_from_cpp() {
    std::cout << "Extern CPP: Hello from C++!" << std::endl;
}

int main() {
    std::cout << "Hello from C++!" << std::endl;
    rust_function();
    return 0;
}
