use std::{io, thread, time::Duration};

use crossterm::{
    event::{poll, read, KeyCode, KeyEvent},
    execute,
    style::{Color, Colors, Print, ResetColor, SetColors},
    terminal::{
        self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
    QueueableCommand,
};
use rand::Rng;

const APPLE_COLOUR: Color = Color::DarkRed;
const BG_COLOUR: Color = Color::Cyan;
const SNAKE_COLOUR: Color = Color::Black;

fn print_cell(fg: Color, bg: Color, content: &str) -> io::Result<()> {
    execute!(
        io::stdout(),
        SetColors(Colors::new(fg, bg)),
        Print(content.to_string()),
        ResetColor
    )
}

fn rand_value(max: u16) -> usize {
    rand::thread_rng().gen_range(1..max - 1) as usize
}

struct State {
    game_over: bool,
    board: Vec<Vec<Color>>,
    body: Vec<(i32, i32)>,
    direction: (i32, i32),
}

impl State {
    fn new() -> Self {
        let term_size = crossterm::terminal::size().unwrap();
        let mut state = State {
            game_over: false,
            board: Vec::new(),
            body: Vec::new(),
            direction: (1, 0),
        };
        for x in 0..term_size.1 - 1 {
            state.board.push(Vec::new());

            for _ in 0..term_size.0 - 1 {
                state.board[x as usize].push(BG_COLOUR);
            }
        }

        let apple_x = rand_value(term_size.1);
        let apple_y = rand_value(term_size.0);
        state.board[apple_x][apple_y] = APPLE_COLOUR;
        state.board[4][2] = SNAKE_COLOUR;
        state.body.push((0, 2));
        state.direction = (1, 0);

        state
    }

    fn update(&mut self) {
        let term_size = crossterm::terminal::size().unwrap();
        let mut new = self.board.clone();
        let mut apple_cell = (0, 0);

        for idx in 0..self.board.len() {
            for idy in 0..self.board[idx].len() {
                // move body
                for idz in 0..self.body.len() {
                    if self.body[idz] == (idx as i32, idy as i32) {
                        let mut new_x = idx as i32 + self.direction.0;
                        if new_x == term_size.1.into() {
                            new_x = 1;
                        } else if new_x < 1 {
                            new_x += (term_size.1 - 1) as i32;
                        }

                        let mut new_y = idy as i32 + self.direction.1;
                        if new_y == term_size.0.into() {
                            new_y = 1;
                        } else if new_y < 0 {
                            new_y += (term_size.0 - 1) as i32;
                        }

                        new[(new_x - 1) as usize][new_y as usize] = SNAKE_COLOUR;
                        new[idx][idy] = BG_COLOUR;

                        // update body vec
                        self.body[idz] = (new_x as i32, new_y as i32);
                    }
                }

                if self.board[idx][idy] == APPLE_COLOUR {
                    apple_cell = (idx, idy);
                } else {
                    new[idx][idy] = BG_COLOUR;
                }
            }
        }

        // check if apple is consumed
        if new[apple_cell.0][apple_cell.1] == APPLE_COLOUR {
            let apple_x = rand_value(term_size.1);
            let apple_y = rand_value(term_size.0);
            new[apple_x][apple_y] = APPLE_COLOUR;

            //append new snake body
            let last_cell = self.body.last().unwrap();
            self.body.push((
                last_cell.0 - self.direction.0 as i32,
                last_cell.1 - self.direction.1 as i32,
            ));
        }

        self.board = new;
    }

    fn display(&self) -> io::Result<()> {
        terminal::Clear(terminal::ClearType::All);
        io::stdout().queue(crossterm::cursor::MoveTo(0, 0))?;

        for idx in 0..self.board.len() {
            for idy in 0..self.board[idx].len() {
                let colour = &self.board[idx][idy];

                print_cell(Color::White, *colour, " ")?;
            }
            print_cell(Color::White, Color::Black, "\r\n")?;
        }

        Ok(())
    }

    fn read_input(&mut self) {
        let poll_ret = poll(Duration::from_secs(0));
        if let Ok(i) = poll_ret {
            if i {
                let event = read().unwrap();
                match event {
                    crossterm::event::Event::Key(KeyEvent {
                        code: KeyCode::Left,
                        ..
                    }) => {
                        self.direction = (-1, 0);
                    }
                    crossterm::event::Event::Key(KeyEvent {
                        code: KeyCode::Up, ..
                    }) => {
                        self.direction = (0, -1);
                    }
                    crossterm::event::Event::Key(KeyEvent {
                        code: KeyCode::Down,
                        ..
                    }) => {
                        self.direction = (0, 1);
                    }
                    crossterm::event::Event::Key(KeyEvent {
                        code: KeyCode::Right,
                        ..
                    }) => {
                        self.direction = (1, 0);
                    }
                    crossterm::event::Event::Key(KeyEvent {
                        code: KeyCode::Char('q'),
                        ..
                    }) => {
                        self.game_over = true;
                    }
                    _ => {}
                }
            }
        }
    }
}

fn main() -> io::Result<()> {
    enable_raw_mode().unwrap();
    execute!(io::stdout(), EnterAlternateScreen)?;

    let mut state = State::new();
    loop {
        if state.game_over {
            break;
        }

        state.display()?;
        state.read_input();
        state.update();

        thread::sleep(Duration::from_millis(1000));
    }

    execute!(io::stdout(), LeaveAlternateScreen)?;
    disable_raw_mode().unwrap();
    println!("Game over. Final score was {}", state.body.len());
    Ok(())
}
