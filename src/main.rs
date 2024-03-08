use core::time;
use std::{
    io::{self, Write},
    time::Duration, thread,
};

use snek::{
    state::{Command, Direction, State},
    utils::{first_food, print_at_cell}
};

use crossterm::{
    event::{poll, read, KeyCode, KeyEvent},
    execute,
    terminal::{
        self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    }, ExecutableCommand, style::Stylize,
};

fn check_input() -> Option<Command> {
    let poll_ret = poll(Duration::from_secs(0));
    if let Ok(i) = poll_ret {
        if i {
            let event = read().unwrap();
            match event {
                crossterm::event::Event::Key(KeyEvent {
                    code: KeyCode::Left,
                    ..
                }) => return Some(Command::Go(Direction::Left)),
                crossterm::event::Event::Key(KeyEvent {
                    code: KeyCode::Down,
                    ..
                }) => return Some(Command::Go(Direction::Down)),
                crossterm::event::Event::Key(KeyEvent {
                    code: KeyCode::Up, ..
                }) => return Some(Command::Go(Direction::Up)),
                crossterm::event::Event::Key(KeyEvent {
                    code: KeyCode::Right,
                    ..
                }) => return Some(Command::Go(Direction::Right)),
                crossterm::event::Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    ..
                }) => return Some(Command::Quit),
                _ => return None,
            }
        };
    };

    None
}

fn main() -> io::Result<()> {
    enable_raw_mode().unwrap();
    execute!(io::stdout(), EnterAlternateScreen)?;
    io::stdout().execute(terminal::Clear(terminal::ClearType::All))?;

    print_at_cell(&(6, 5), "_".reverse()).unwrap();

    io::stdout().flush()?;
    let mut state = State{ body_cells: vec![], food_cell: first_food(), direction: Direction::Right };

    loop {
        // CHeck if head on food 
            // delete food, increase cells, generate new food
        // move snake in new direction

        let input = check_input();
        if let Some(cmd) = input {
            match cmd {
                Command::Go(dir) => { state.direction = dir; },
                Command::Quit => break,
            }
        }
        thread::sleep(Duration::from_millis(100));
    }

    execute!(io::stdout(), LeaveAlternateScreen)?;
    disable_raw_mode().unwrap();

    Ok(())
}
