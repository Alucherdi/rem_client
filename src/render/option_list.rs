use std::io::stdout;

use crossterm::{execute, style::Print};

pub struct OptionList {

}

impl OptionList {
    pub fn render_options() {
        for _i in 0..10 {
            execute!(
                stdout(),
                Print(
                    format!("{} {}", "  >", "Hi\r\n")
                )
            ).unwrap();
        }
    }
}
