pub enum Direction { Left, Up, Down, Right }
pub enum Command { Go(Direction), Quit, }

pub struct State {
    pub body_cells: Vec<(u16, u16)>,
    pub food_cell: (u16, u16),
    pub direction: Direction
}

impl State {
    pub fn print() {
        // TODO: Use print_to_cell to print out the snake and food cell
    }
}
