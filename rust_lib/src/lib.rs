// This is needed so the Rust compiler know that this function exists 
// and that it will be provided while runtime.
extern "C" {
    fn hello_from_cpp(y: f64);
}

#[no_mangle]
pub extern "C" fn rust_function(x: i32) {
    println!("Hello from Rust! Received: {}", x);
    unsafe { hello_from_cpp(3.14) };
}
