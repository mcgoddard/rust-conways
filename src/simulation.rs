use std::error::Error;
use std::fs;
use std::io::prelude::*;
use std::path::Path;
use std::thread;

#[derive(Clone)]
#[derive(Debug)]
pub enum CellState
{
	Dead,
	Alive
}

pub struct Simulator {
	iteration_num: u32,
	height: usize,
	width: usize,
	output_dir: Option<String>,
	starting_states: Vec<Vec<CellState>>,
	current_iteration: u32,
}

impl<'a> Simulator {
	pub fn new(iteration_num: u32, output_dir: Option<String>, 
		starting_states: Vec<Vec<CellState>>) -> Simulator {
		// Check input data
		if starting_states.len() < 1 {
			panic!("not enough rows");
		}
		let row_length = starting_states[0].len();
		for row in &starting_states {
			if row.len() != row_length {
				panic!("row lengths do not match");
			}
		}
		// Check input directory
		match output_dir {
			Some(ref o) => {
				if !fs::metadata(&o).is_ok() {
					match fs::create_dir_all(&o) {
						Ok(_) => {},
						Err(err) => {
							panic!("output directory did not exist and cannot be created: {}",
								err.description());
						}
					}
				}
			},
			None => {}
		}
		// Assign struct
		Simulator {
			iteration_num: iteration_num,
			height: starting_states.len(),
			width: starting_states[0].len(),
			output_dir: output_dir,
			starting_states: starting_states,
			current_iteration: 0,
		}
	}

	pub fn run_simulation(&mut self) {
		// Set up first cells
		let mut current_states = self.create_initial_states();
		loop {
			let mut threads = Vec::with_capacity(self.height);
			// Create new states
			{
				// Spawn threads to calculate next states
				for x in 0..self.height {
					// Capture states immutably
					let states = current_states.clone();
					let row = states[x].clone();
					let width = self.width.clone();
					threads.push(thread::spawn(move || {
						let mut new_row = Vec::with_capacity(width);
						for y in 0..width {
							let cell = row[y].clone();
							let new_cell = cell.iterate(&states);
							new_row.push(new_cell);
						}
						return new_row;
					}));
				}
			}
			current_states = Vec::with_capacity(self.height);
			for child in threads {
				current_states.push(match child.join() {
					Ok(row) => row,
					Err(err) => {
						panic!("Couldn't compute row: TODO get and print error msg")
					}
				});
			}
			// Output
			match self.output_dir {
				Some(ref o) => {
					let current_states = &current_states;
					self.output(o, current_states);
				},
				None => {}
			}
			// Increment iteration
			self.current_iteration += 1;
			if self.current_iteration == self.iteration_num {
				break;
			}
		}
	}

	fn create_initial_states(&mut self) -> Vec<Vec<Cell>> {
		let mut states = Vec::with_capacity(self.height);
		for x in 0..self.starting_states.len() {
			let mut row = Vec::with_capacity(self.width);
			for y in 0..self.starting_states[x].len() {
				row.push(Cell {
					state: self.starting_states[x][y].clone(),
					row: x,
					col: y,
				});
			}
			states.push(row);
		}
		return states;
	}

	fn output(& self, output_dir: &String, current_states: &Vec<Vec<Cell>>) {
		let path = Path::new(&output_dir);
		let path = path.join(format!("{}.csv", self.current_iteration));
		let mut file = match fs::File::create(&path) {
			Err(why) => panic!("couldn't create {}: {}",
								path.display(), why.description()),
			Ok(file) => file,
		};
		for row in current_states {
			let mut row_str: String = row.iter().map(|c| match c.state {
				CellState::Dead => "0",
				CellState::Alive => "1",
			}).collect::<Vec<&str>>().join(",");
			row_str.push('\n');
			match file.write_all(row_str.as_bytes()) {
				Err(why) => {
					panic!("couldn't write to {}: {}", path.display(),
						   why.description())
				},
				Ok(_) => {},
			}
		}
	}
}

#[derive(Clone)]
struct Cell {
	state: CellState,
	row: usize,
	col: usize,
}

impl Cell {
	fn iterate(self, prev_state: &Vec<Vec<Cell>>) -> Cell {
		let alive_neighbours = self.count_alive_neighbours(prev_state);
		return Cell {
			state: self.conways_rules(alive_neighbours),
			row: self.row,
			col: self.col,
		};
	}

	fn count_alive_neighbours(&self, prev_state: &Vec<Vec<Cell>>) -> usize {
		let mut alive_neighbours: usize = 0;
		for x in 0..3 {
			for y in 0..3 {
				if (x != 1 || y != 1) &&
					((((self.row > 0) && ((self.row + 1) < prev_state.len())) ||
					(((self.row == 0) && (x > 0)) || ((self.row + 1) == prev_state.len() && (x < 2)))) &&
					(((self.col > 0) && (self.col + 1 < prev_state[0].len())) ||
					(((self.col == 0) && (y > 0)) || (((self.col + 1) == prev_state[0].len()) && (y < 2))))) {
					let n_x: usize = self.row + x - 1;
					let n_y: usize = self.col + y - 1;
					alive_neighbours = match prev_state[n_x][n_y].state {
						CellState::Alive => alive_neighbours + 1,
						CellState::Dead => alive_neighbours
					}
				}
			}
		}
		return alive_neighbours;
	}

	fn conways_rules(&self, alive_neighbours: usize) -> CellState {
		match self.state {
			CellState::Alive => {
				if alive_neighbours < 2 || alive_neighbours > 3 {
					CellState::Dead
				} else {
				    CellState::Alive
				}
			},
			CellState::Dead => {
				if alive_neighbours == 3 {
					CellState::Alive
				} else {
				    CellState::Dead
				}
			}
		}
	}
}