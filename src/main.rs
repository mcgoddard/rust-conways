mod simulation;

extern crate getopts;
extern crate rand;

use std::env;
use std::str;
use getopts::Options;
use rand::Rng;
use simulation::CellState;
use simulation::Simulator;

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
	// Check for help request
	if matches.opt_present("h") {
		print_usage(&program, opts);
		return;
	}
	// Parse number of iterations to run
	let iterations_str = match matches.opt_str("n") {
		Some(s) => { s }
		None => {
			println!("Required parameter 'iterations' missing");
			print_usage(&program, opts);
			return;
		}
	};
	let iterations = match iterations_str.parse::<u32>() {
		Ok(i) => i,
		Err(err) => {
			println!("Invalid value for iterations (flag 'i'):\n{}\n", err);
			return;
		}
	};
	// Parse output dir
	let output = if !matches.free.is_empty() {
		matches.free[0].clone()
	} else {
		println!("Required argument OUTPUT_DIR missing\n");
		print_usage(&program, opts);
		return;
	};
	// TODO: Determine if using input file or random grid
	// Handle random starting grid
	// Parse height and width of grid
	let height_str = match matches.opt_str("t") {
		Some(expr) => expr,
		None => String::from("None"),
	};
	let height: usize = match height_str.parse::<usize>() {
		Ok(h) => h,
		Err(err) => {
			println!("Invalid value for height (flag 't'):\n{}\n", err);
			return;
		}
	};
	let width_str = match matches.opt_str("w") {
		Some(expr) => expr,
		None => String::from("None"),
	};
	let width: usize = match width_str.parse::<usize>() {
		Ok(w) => w,
		Err(err) => {
			println!("Invalid value for width (flag 'w'):\n{}\n", err);
			return;
		}
	};
	// Generate grid
	let mut grid = Vec::new();
	// Generation random grid
	for _ in 0..height {
		let mut row = Vec::new();
		for _ in 0..width {
			let rn = rand::thread_rng().gen_range(0, 2);
			let cell = match rn {
				0 => CellState::Dead,
				1 => CellState::Alive,
				_ => panic!("Somehow got a incorrect random number!?")
			};
			row.push(cell);
		}
		grid.push(row);
	}
	// Run simulation
	let mut sim = Simulator::new(iterations, output, grid);
	sim.run_simulation();
}
