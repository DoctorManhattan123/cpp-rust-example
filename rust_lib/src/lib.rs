// This includes the automatically generated bindings (generated with bindgen)
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/generated/bindings.rs"));

#[no_mangle]
pub extern "C" fn rust_function() {
    println!("Hello from Rust!");
    unsafe {
        hello_from_cpp(3.14);
        another_cpp_function(42);
    }
}
