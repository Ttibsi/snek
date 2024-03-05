use std::io;

use crossterm::{
    QueueableCommand,
    style::{self, StyledContent},
    terminal,
};

pub fn first_food() -> (u16, u16) {
    let mut ret = (0,0);

    let term_size = terminal::size();
    if let Ok(size) = term_size {
        ret.0 = (size.0 * 2) / 3;
        ret.1 = (size.1 * 2) / 3;
    }

    ret
}

pub fn print_at_cell<T: std::fmt::Display>(location: &(u16, u16), c: StyledContent<T>) -> io::Result<()> {
    let a = location.0;
    let b = location.1;

    io::stdout()
        .queue(crossterm::cursor::MoveTo(a, b))?
        .queue(style::PrintStyledContent(c))?;

    Ok(())
}
