use std::{
    io::{self, Write},
    time::Duration,
};

use snek::{
    state::{Command, Direction, State},
    utils::first_food
};

use crossterm::{
    event::{poll, read, KeyCode, KeyEvent},
    execute,
    terminal::{
        self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
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
    let _ = enable_raw_mode();
    execute!(io::stdout(), EnterAlternateScreen)?;
    io::stdout().execute(terminal::Clear(terminal::ClearType::All))?;

    print_at_cell((6, 5), "_".reverse());

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
        // smol sleep 
    }

    execute!(io::stdout(), LeaveAlternateScreen)?;
    let _ = disable_raw_mode();

    Ok(())
}
