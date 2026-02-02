use crossterm::{
    QueueableCommand,
    cursor::MoveTo,
    event::{Event, KeyCode, read},
    style::Print,
    terminal::{Clear, ClearType},
};
use std::io::{self, Write};

pub struct Game {}

impl Game {
    pub fn new() -> Self {
        Game {}
    }

    pub fn run(&mut self) -> io::Result<()> {
        let mut stdout = io::stdout();

        let mut x = 0;
        let mut y = 0;

        loop {
            // draw
            stdout.queue(Clear(ClearType::All))?;
            stdout.queue(MoveTo(x, y))?;
            stdout.queue(Print('@'))?;
            stdout.flush()?;

            // input
            match read()? {
                Event::Key(key_event) => {
                    match key_event.code {
                        KeyCode::Esc => break,

                        KeyCode::Left => {
                            if x > 0 {
                                x -= 1
                            }
                        }
                        KeyCode::Right => x += 1,
                        KeyCode::Up => {
                            if y > 0 {
                                y -= 1
                            }
                        }
                        KeyCode::Down => y += 1,

                        _ => {}
                    };
                }
                _ => {}
            }
        }

        Ok(())
    }
}
