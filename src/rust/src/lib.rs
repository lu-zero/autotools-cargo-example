
#[no_mangle]
/// This is an example function
///
/// A simple hello world
pub extern "C" fn hello_world() {
    println!("hello world! -> from rust!");
}
