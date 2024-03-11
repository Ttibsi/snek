use std::io::{self, Write};

use crossterm::{
    style::{self, StyledContent},
    QueueableCommand,
};

pub fn first_food(term_size: (u16, u16)) -> (u16, u16) {
    ((term_size.0 * 2) / 3, (term_size.1 * 2) / 3)
}

pub fn print_at_cell<T: std::fmt::Display>( location: &(u16, u16), c: StyledContent<T>,) -> io::Result<()> {
    let a = location.0;
    let b = location.1;

    io::stdout()
        .queue(crossterm::cursor::MoveTo(a, b))?
        .queue(style::PrintStyledContent(c))?;

    io::stdout().flush()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_food() {
        assert_eq!(first_food((12, 12)), (8,8));
    }
}

