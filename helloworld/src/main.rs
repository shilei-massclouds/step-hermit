fn main() {
    println!("Hello, monk!");
}

// Dummy
// Todo: move it to libax
#[no_mangle]
pub extern "C" fn sys_network_init() -> i32 {
	// nothing to do
	0
}