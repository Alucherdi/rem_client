pub mod event;
pub mod key_handler;
pub mod app;
pub mod render;

use app::App;
use crossterm::terminal;

fn main() {
    let mut app = App{};

    terminal::enable_raw_mode().unwrap();

    match app.run() {
        Ok(_) => (),
        Err(e) => println!("Error: {}", e),
    };

    terminal::disable_raw_mode().unwrap();
}
