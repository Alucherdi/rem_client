use std::io::stdout;

use crossterm::{execute, style::Print};

use self::option_list::OptionList;

mod option_list;

pub struct Render {
    pub selection: i16,
    pub url: String,
}

impl Render {
    pub fn new() -> Render {
        Render {
            selection: 0,
            url: "".to_string(),
        }
    }

    pub fn draw(&self) {
        execute!(
            stdout(),
            Print(
                format!("{}\r\n", self.url)
            )
        ).unwrap();

    }

    pub fn scroll() {
    }
}
