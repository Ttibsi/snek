use std::io;

use crossterm::{terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, execute, style::Stylize};

fn main() -> io::Result<()> {
    let _ = enable_raw_mode();
    execute!(io::stdout(), EnterAlternateScreen)?;

    crossterm::cursor::MoveTo(12, 11);
    print!("{}", " ".reverse());

    execute!(io::stdout(), LeaveAlternateScreen)?;
    let _ = disable_raw_mode();

    Ok(())
}

