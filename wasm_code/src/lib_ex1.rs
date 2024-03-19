#![allow(unused_variables)]

use std::{
	fs,
	io::Bytes,
	time::{Duration, SystemTime},
};
/// Takes in a simple primitive signed 32-bit integer
/// and adds one to it, returning the result.
///
/// It is ready for wasm execution thanks to the `#[no_mangle]` attribute.
/// You may find that you also need `extern "c"`.
#[no_mangle]
pub extern "C" fn add_one(n: i32) -> i32 {
	n.saturating_add(1)
}

/// Prints the number 42 to the terminal.
/// It does not do any math, the number is always 42.
///
/// Reminder, you may need to add some annotations to this function's declaration
/// to make it work with wasm.
#[no_mangle]
pub fn print_forty_two() {
	println!("{}", 42);
}

/// Calculates the division between two integers number
#[no_mangle]
pub fn div(a: i32, b: i32) -> i32 {
	a / b
}

/// Fetches the current system time and determines whether it is after the year 2000
/// Returns true iff the current time is after January 1st 2000
/// you may find https://doc.rust-lang.org/std/time/struct.SystemTime.html useful
#[no_mangle]
pub fn wen_millennium() -> bool {
	SystemTime::now() > SystemTime::UNIX_EPOCH + Duration::from_secs(946684800)
}

/// Calculates the sum of two floating point numbers
#[no_mangle]
pub fn sum_floats(a: f32, b: f32) -> f32 {
	a + b
}

/// Takes in a simple primitive signed 32-bit integer
/// and adds one to it, and writes the result to a file.
#[no_mangle]
pub fn write_to_file(n: i32) {
	// This can be made to work
	// See `wasmtime run --help` for a clue
	let result = n.saturating_add(1);
	// write result in a file
	fs::write("result.txt", result.to_string()).unwrap()
}
