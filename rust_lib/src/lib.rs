// This is needed so the Rust compiler know that this function exists 
// and that it will be provided while runtime.
extern "C" {
    fn hello_from_cpp();
}

#[no_mangle]
pub extern "C" fn rust_function() {
    println!("Hello from Rust!");
    unsafe { hello_from_cpp() };
}

