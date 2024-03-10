use std::{
    io,
    thread,
    time::Duration,
    collections::HashMap,
};

use snek::{
    state::{Command, Direction, State},
    utils::first_food,
};

use crossterm::{
    event::{poll, read, KeyCode, KeyEvent},
    execute,
    terminal::{
        self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
    ExecutableCommand,
};

fn check_input(s: &mut State) -> Option<Command> {
    let poll_ret = poll(Duration::from_secs(0));
    if let Ok(i) = poll_ret {
        if i {
            let event = read().unwrap();
            match event {
                crossterm::event::Event::Key(KeyEvent {code: KeyCode::Left, ..}) => {
                    s.direction_change.insert(Direction::Left, s.body_cells[0]);
                    return Some(Command::Go(Direction::Left));
                }
                crossterm::event::Event::Key(KeyEvent { code: KeyCode::Down, .. }) => {
                    s.direction_change.insert(Direction::Down, s.body_cells[0]);
                    return Some(Command::Go(Direction::Down));
                }
                crossterm::event::Event::Key(KeyEvent { code: KeyCode::Up, .. }) => {
                    s.direction_change.insert(Direction::Up, s.body_cells[0]);
                    return Some(Command::Go(Direction::Up));
                }
                crossterm::event::Event::Key(KeyEvent { code: KeyCode::Right, .. }) => {
                    s.direction_change.insert(Direction::Right, s.body_cells[0]);
                    return Some(Command::Go(Direction::Right));
                }
                crossterm::event::Event::Key(KeyEvent { code: KeyCode::Char('q'), .. }) => {
                    return Some(Command::Quit);
                }
                _ => return None,
            }
        };
    };

    None
}

fn main() -> io::Result<()> {
    enable_raw_mode().unwrap();
    execute!(io::stdout(), EnterAlternateScreen)?;

    let term_size = crossterm::terminal::size().unwrap();
    let mut state = State {
        body_cells: vec![(term_size.0, 1)],
        food_cell: first_food(),
        direction: Direction::Right,
        score: 0,
        direction_change: HashMap::new()
    };

    loop {
        io::stdout().execute(terminal::Clear(terminal::ClearType::All))?;
        state.print();

        if state.body_cells[0] == state.food_cell.clone() {
            state.new_food(&term_size);
        }

        // TODO: check collision
        state.move_snake(&term_size);

        let input = check_input(&mut state);
        if let Some(cmd) = input {
            match cmd {
                Command::Go(dir) => {
                    state.direction = dir;
                }
                Command::Quit => break,
            }
        }

        thread::sleep(Duration::from_millis(50));
    }

    execute!(io::stdout(), LeaveAlternateScreen)?;
    disable_raw_mode().unwrap();

    Ok(())
}
