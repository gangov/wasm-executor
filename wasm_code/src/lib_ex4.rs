#![allow(unused_variables)]
/// ATTENTION: Before you can use this code, you must change the path in `Cargo.toml`

// Here we define the signatures of the Host Functions described in the Readme
// extern "C" lets us define the signatures of the host functions
// without requiring us to implement them
//
// Those function will be translated to "Imports" in wasm,
// and provided by the executor
extern "C" {
	fn get() -> u32;
	fn set(val: u32);
}

// One inconvenience with the host function definition above is that
// we have to use the `unsafe` keyword each time we call them.

/// A wrapper around the `set` host function to avoid writing
// every time 'unsafe'
fn set_hf(val: u32) {
	unsafe {
		set(val);
	}
}

//TODO also implement get_hf
fn get_hf() -> u32 {
	unsafe { get() }
}

/// Implement the entry point described in the Readme.
/// The logic may do whatever you want, but you will only be able to
/// observe it if you write the result to the shared state.
///
/// Remember not to change the name of the entry point for the executor will get confused.
#[no_mangle]
fn start() {
	let val = get_hf();
	set_hf(val + 100_000);
}
