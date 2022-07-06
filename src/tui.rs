extern crate termion;

use std::{
    error::Error,
    io,
    sync::mpsc,
    thread,
    time::Duration
};

use termion::{
    event::Key,
    input::TermRead,
    raw::IntoRawMode,
    cursor::DetectCursorPos,
    screen::AlternateScreen,
    {terminal_size, color, style},
};

#[derive(Debug)]
pub enum Event {
    Input(Key),
    Tick,
}

impl Event {
    fn events(tick_rate: Duration) -> mpsc::Receiver<Self> {
        let (tx, rx) = mpsc::channel();

        let key_tx = tx.clone();
        let tick_tx = tx.clone();

        thread::spawn(move || {
            let stdin = io::stdin();

            for key in stdin.keys() {
                if let Err(err) = key_tx.send(Event::Input(key.unwrap())) {
                    eprintln!("{}", err);
                    return;
                }
            }
        });
        thread::spawn(move || loop {
            if let Err(err) = tick_tx.send(Event::Tick) {
                eprintln!("{}", err);
                break;
            }
            thread::sleep(tick_rate);
        });

        rx
    }
}

pub type InputCallback = fn(Key);
pub type TickCallback = fn();

pub struct Tui {
    stdout: AlternateScreen<termion::raw::RawTerminal<std::io::Stdout>>,
    on_input: InputCallback,
    on_tick: TickCallback,
}

impl Tui {
    pub fn new(on_input: InputCallback, on_tick: TickCallback) -> Self {

        Self {
            stdout: AlternateScreen::from(io::stdout().into_raw_mode().unwrap()),
            on_input,
            on_tick,
        }
    }

    pub fn run(self) {
        let rx = Event::events(Duration::from_millis(16));

        loop {
            match rx.recv().unwrap() {
                Event::Input(key) => match key {
                    Key::Char('q') => break,
                    _ => (self.on_input)(key),
                },
                Event::Tick => (self.on_tick)(),
            }
            // if app.should_quit {
            //     return Ok(());
            // }
        }
    }
}
