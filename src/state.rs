use crate::utils::print_at_cell;
use rand::Rng;
use crossterm::style::Stylize;

pub enum Direction { Left, Up, Down, Right }
pub enum Command { Go(Direction), Quit, }

pub struct State {
    pub body_cells: Vec<(u16, u16)>,
    pub food_cell: (u16, u16),
    pub direction: Direction,
    pub score: i32
}

impl State {
    pub fn print(self) {
        self.body_cells.iter().for_each(|elem| print_at_cell(elem, 'o'.reverse()).unwrap());
        print_at_cell(&self.food_cell, '*'.reverse()).unwrap();
    }

    pub fn new_food(&mut self, term_size:&(u16, u16) ) {
        self.score += 1;
        self.food_cell = (
            rand::thread_rng().gen_range(1..term_size.0 - 1),
            rand::thread_rng().gen_range(1..term_size.1 - 1)
        );
     
        // TODO: add new cell

    }

    pub fn move_snake(&mut self) { 
        for cell in &mut self.body_cells {
            *cell = match self.direction {
                Left => (cell.0, cell.1 - 1),
                Up  => (cell.0, cell.1 - 1),
                Down => (cell.0, cell.1 - 1),
                Right => (cell.0, cell.1 - 1),
            };
        };
    }
}
