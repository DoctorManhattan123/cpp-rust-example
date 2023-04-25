// This includes the automatically generated bindings (generated with bindgen)
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/generated/bindings.rs"));

#[no_mangle]
pub extern "C" fn rust_function(x: i32) {
    println!("Hello from Rust! Received: {}", x);
    unsafe { hello_from_cpp(3.14) };
}
