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
	output_dir: String,
	starting_states: Vec<Vec<CellState>>,
	current_iteration: u32,
}

impl<'a> Simulator {
	pub fn new(iteration_num: u32, output_dir: String, 
		starting_states: Vec<Vec<CellState>>) -> Simulator {
		// TODO: modify so starting_states dimensions are correct
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
		let mut current_states = Vec::new();
		for x in 0..self.starting_states.len() {
			let mut row = Vec::new();
			for y in 0..self.starting_states[x].len() {
				row.push(Cell {
					state: self.starting_states[x][y].clone(),
					row: x,
					col: y,
				});
			}
			current_states.push(row);
		}
		loop {
			// Create new states
			let mut new_states = Vec::new();
			for x in 0..self.height {
				let mut new_row = Vec::new();
				for y in 0..self.width {
					let cell = current_states[x][y].clone();
					let new_cell = cell.iterate(&current_states);
					new_row.push(new_cell);
				}
				new_states.push(new_row);
			}
			// Set current states
			current_states = new_states;
			// TODO Output
			// Increment iteration
			self.current_iteration += 1;
			if self.current_iteration == self.iteration_num {
				break;
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
		return Cell {
			state: self.state,
			row: self.row,
			col: self.col,
		};
	}
}