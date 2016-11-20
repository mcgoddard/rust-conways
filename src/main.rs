mod simulation;

extern crate getopts;
extern crate rand;
extern crate stopwatch;

use std::env;
use std::str;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use getopts::Options;
use rand::Rng;
use simulation::CellState;
use simulation::Simulator;
use stopwatch::{Stopwatch};

fn print_usage(program: &str, opts: Options) {
	let brief = format!("Usage: {} OUTPUT_DIR [options]", program);
	print!("{}", opts.usage(&brief));
}

fn generate_grid(height: usize, width: usize) -> Vec<Vec<CellState>> {
	let mut grid = Vec::new();
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
	return grid;
}

fn parse_input_file(path: &Path) -> Vec<Vec<CellState>> {
	match File::open(&path) {
		Ok(file) => {
			let input_file = BufReader::new(file);
			let mut grid: Vec<Vec<CellState>> = Vec::new();
			for line in input_file.lines().filter_map(|result| result.ok()) {
				let chars = line.split(",");
				let mut row: Vec<CellState> = Vec::new();
				for cell in chars {
					let c: CellState = match cell {
						"0" => CellState::Dead,
						"1" => CellState::Alive,
						_ => panic!("Invalid value in input file")
					};
					row.push(c);
				}
				grid.push(row);
			}
			grid
		},
		Err(e) => {
			panic!("Could not open input file: {}", e);
		}
	}
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
	opts.optopt("i", "input_file", "Provide a file containing the initial state",
		"INPUT_FILE");
	opts.optopt("o", "output_dir", "Provide a directory for output", "OUTPUT_DIR");

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
	let output = matches.opt_str("o");
	// Create starting states
	let grid: Vec<Vec<CellState>> = match matches.opt_str("i") {
		Some(i) => {
			// An input file is specified
			let path = Path::new(&i);
			parse_input_file(path)
		},
		None => {
			// Parse height and width of grid
			let height_str = match matches.opt_str("t") {
				Some(expr) => expr,
				None => String::from("None"),
			};
			let height: usize = match height_str.parse::<usize>() {
				Ok(h) => h,
				Err(err) => panic!("Invalid value for height (flag 't'):\n{}\n", err)
			};
			let width_str = match matches.opt_str("w") {
				Some(expr) => expr,
				None => String::from("None"),
			};
			let width: usize = match width_str.parse::<usize>() {
				Ok(w) => w,
				Err(err) => panic!("Invalid value for width (flag 'w'):\n{}\n", err)
			};
			// Generation random grid
			generate_grid(height, width)
		}
	};
	// Run simulation
	let mut sim = Simulator::new(iterations, output, grid);
	let sw = Stopwatch::start_new();
	sim.run_simulation();
	println!("Time taken: {}", sw.elapsed_ms());
}
