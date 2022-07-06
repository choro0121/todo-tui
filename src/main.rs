// https://docs.rs/termion
// https://github.com/redox-os/termion

// https://qiita.com/hatoo@github/items/905a19a98876e7446edf
// https://github.com/hatoo/kiro

mod task;
mod parser;
mod tui;

use termion::event::Key;

fn type_of<T>(_: T) -> String{
    let a = std::any::type_name::<T>();
    return a.to_string();
}

fn on_input(key: Key) {
    println!("{:?}", key)
}

fn on_tick() {
    println!("tick!")
}

struct App {
    count: u16,
}

impl App {
    pub fn new() -> Self {
        Self {
            count: 0
        }
    }

    pub fn run(&mut self) {
        let tui = tui::Tui::new(on_input, on_tick);
        let rx = tui.run();
    }
}

fn main() {
    App::new().run()
}