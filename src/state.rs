use crate::utils::print_at_cell;
use crossterm::style::Stylize;
use rand::Rng;

pub enum Direction {
    Left,
    Up,
    Down,
    Right,
}
pub enum Command {
    Go(Direction),
    Quit,
}

pub struct State {
    pub body_cells: Vec<(u16, u16)>,
    pub food_cell: (u16, u16),
    pub direction: Direction,
    pub score: i32,
}

impl State {
    pub fn print(&self) {
        self.body_cells
            .iter()
            .for_each(|elem| print_at_cell(elem, 'o'.reverse()).unwrap());
        print_at_cell(&self.food_cell, '*'.reverse()).unwrap();
    }

    pub fn new_food(&mut self, term_size: &(u16, u16)) {
        self.score += 1;
        self.food_cell = (
            rand::thread_rng().gen_range(1..term_size.0 - 1),
            rand::thread_rng().gen_range(1..term_size.1 - 1),
        );

        if let Some(c) = self.body_cells.last() {
            let last_cell = c;

            match self.direction {
                Direction::Left => {self.body_cells.push((last_cell.0 + 1, last_cell.1))},
                Direction::Up => {self.body_cells.push((last_cell.0, last_cell.1 + 1))},
                Direction::Down => {self.body_cells.push((last_cell.0, last_cell.1 - 1))},
                Direction::Right => {self.body_cells.push((last_cell.0 - 1, last_cell.1))},
            }
        }
    }

    // TODO: We need to work out where the direction changes and move the body 
    // cells along that path
        // Potentially we need to add a "direction change" location to the state
    pub fn move_snake(&mut self, term_size: &(u16, u16)) {
        for cell in &mut self.body_cells {
            *cell = match &self.direction {
                Direction::Left => {
                    if cell.0 > 1 {
                        (cell.0 - 1, cell.1)
                    } else {
                        (term_size.0, cell.1)
                    }
                },
                Direction::Up => {
                    if cell.1 > 1 {
                        (cell.0, cell.1 - 1)
                    } else {
                        (cell.0, term_size.1)
                    }
                },
                Direction::Down => {
                    if cell.1 < term_size.1 {
                        (cell.0, cell.1 + 1)
                    } else {
                        (cell.0, 1)
                    }
                }
                Direction::Right => {
                    if cell.0 < term_size.0 {
                        (cell.0 + 1, cell.1)
                    } else {
                        (1, cell.1) 
                    }
                }
            }
        }
    }
}
