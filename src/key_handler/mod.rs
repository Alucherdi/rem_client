use std::time::Duration;
use crossterm::event::{Event, self, KeyCode, KeyEvent};

use crate::event::AppEvent;

pub fn read_key() -> Result<AppEvent, std::io::Error> {
    let mut key_event = AppEvent::Non;

    if event::poll(Duration::from_millis(500))? {
        if let Event::Key(event) = event::read()? {
            key_event = match event {
                KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: event::KeyModifiers::CONTROL,
                    ..
                } | KeyEvent {
                    code: KeyCode::Esc,
                    ..
                }=> AppEvent::End,
                _ => {
                    match event.code {
                        KeyCode::Char(c) => AppEvent::SendKey(c),
                        _ => AppEvent::Non
                    }
                }
            };
        }
    }

    Ok(key_event)
}
