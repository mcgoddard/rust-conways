#[derive(Clone)]
#[derive(Debug)]
pub enum CellState
{
	Dead,
	Alive
}

pub struct Simulator<'a> {
	iterationNum: u32,
	height: u32,
	width: u32,
	output_dir: String,
	starting_states: Vec<Vec<CellState>>,
	current_states: &'a mut Vec<Vec<CellState>>,
	current_iteration: &'a mut u32,
}