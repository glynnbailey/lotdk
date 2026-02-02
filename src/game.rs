use crossterm::{
    QueueableCommand,
    cursor::MoveTo,
    event::KeyCode,
    style::Print,
    terminal::{Clear, ClearType},
};
use std::io::{self, Write};
use crate::input::InputState;

pub struct Game {
    input: InputState,
}

impl Game {
    pub fn new() -> Self {
        Game {
            input: InputState::new()
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        self.draw()?;

        loop {
            // input
            self.input.update()?;

            // update
            if self.input.last_key() == KeyCode::Esc {
                break
            }

            // draw
            self.draw()?;
        }

        Ok(())
    }

    fn draw(&self) -> std::io::Result<()> {
        let mut stdout = io::stdout();
        stdout.queue(Clear(ClearType::All))?;
        stdout.queue(MoveTo(0, 0))?;
        stdout.queue(Print('@'))?;
        stdout.flush()?;
        Ok(())
    }
}
