use crossterm::event::{self, Event, KeyCode};

pub struct InputState {
    last_key: KeyCode,
}

impl InputState {
    pub fn new() -> Self {
        Self { last_key: KeyCode::Null }
    }

    pub fn update(&mut self) -> std::io::Result<()> {
        loop {
            match event::read()? {
                Event::Key(key_event) => {
                    self.last_key = key_event.code;
                    return Ok(());
                }
                _ => continue, // ignore mouse/resize/etc
            }
        }
    }

    pub fn last_key(&self) -> KeyCode {
        self.last_key
    }
}
