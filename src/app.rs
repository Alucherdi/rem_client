use std::io::stdout;

use crossterm::{execute, terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen}, cursor};

use crate::{key_handler::read_key, event::AppEvent, render::Render};

pub struct App {
}

impl App {
    pub fn run(&mut self) -> Result<(), std::io::Error> {
        let mut render = Render::new();
        execute!(stdout(), EnterAlternateScreen)?;

        loop {
            self.clear_screen();
            render.draw();

            match read_key()? {
                AppEvent::End => break,
                AppEvent::SendKey(c) => {
                    render.url.push(c);
                },
                _ => {}
            }
        }

        execute!(stdout(), LeaveAlternateScreen)?;
        Ok(())
    }

    pub fn clear_screen(&self) {
        execute!(stdout(), Clear(ClearType::All)).unwrap();
        execute!(stdout(), cursor::MoveTo(0, 0)).unwrap();
    }
}
