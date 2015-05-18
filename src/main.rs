extern crate getopts;
use getopts::Options
use std::env

fn main() {
	let args : Vec<String> = env::args().collect();
	print!("{:?}", args);

}

// jsoc vw {}
// jsoc vw filename

// jsoc filename
// jsoc {}