extern crate getopts;

use getopts::Options;
use std::env;

fn print_usage(program: &str, opts: Options) {
	let brief = format!("Usage: {} OUTPUT_DIR [options]", program);
	print!("{}", opts.usage(&brief));
}

fn main() {
	// Define args
	let args: Vec<String> = env::args().collect();
	let program = args[0].clone();

	let mut opts = Options::new();
	opts.optflag("h", "help", "Display the help text");
	opts.optopt("w", "width", "Provide a width for the grid", "WIDTH");
	opts.optopt("t", "height", "Provide a height for the grid", "HEIGHT");
	opts.optopt("n", "interations", "Provide the number of iterations to run", 
		"ITERATIONS");

	// Parse args
	let matches = match opts.parse(&args[1..]) {
		Ok(m) => { m }
		Err(f) => { panic!(f.to_string()) }
	};
	if matches.opt_present("h") {
		print_usage(&program, opts);
		return;
	}
	let iterations = match matches.opt_str("n") {
		Some(s) => { s }
		None => {
			println!("Required parameter 'iterations' missing");
			print_usage(&program, opts);
			return;
		}
	};
	let output = if !matches.free.is_empty() {
		matches.free[0].clone()
	} else {
		println!("Required argument OUTPUT_DIR missing\n");
	    print_usage(&program, opts);
	    return;
	};
}
