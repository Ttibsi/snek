use crate::utils::print_at_cell;
use crossterm::style::Stylize;

pub enum Direction { Left, Up, Down, Right }
pub enum Command { Go(Direction), Quit, }

pub struct State {
    pub body_cells: Vec<(u16, u16)>,
    pub food_cell: (u16, u16),
    pub direction: Direction
}

impl State {
    pub fn print(self) {
        self.body_cells.iter().for_each(|elem| print_at_cell(elem, 'o'.reverse()).unwrap());
        print_at_cell(&self.food_cell, '*'.reverse()).unwrap();
    }
}
