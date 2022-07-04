// https://docs.rs/termion
// https://github.com/redox-os/termion

// https://qiita.com/hatoo@github/items/905a19a98876e7446edf
// https://github.com/hatoo/kiro

extern crate termion;

use std::io::{Write, stdout, stdin};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::cursor::DetectCursorPos;
use termion::screen::AlternateScreen;
use termion::{terminal_size, color, style};


mod task;

fn type_of<T>(_: T) -> String{
    let a = std::any::type_name::<T>();
    return a.to_string();
}

enum Mode {
    Normal,
    Command,
}

struct App {
    stdout: AlternateScreen<termion::raw::RawTerminal<std::io::Stdout>>,
    mode: Mode,
    cursor_backup: (u16, u16),
    command: String,
    tasks: Vec<task::Task>
}

impl App {
    pub fn new() -> Self {
        App {
            stdout: AlternateScreen::from(stdout().into_raw_mode().unwrap()),
            mode: Mode::Normal,
            cursor_backup: (0, 0),
            command: "".to_string(),
            tasks: vec![]
        }
    }

    pub fn run(&mut self) {
        write!(self.stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        self.draw_tasks();
        self.stdout.flush().unwrap();
        
        for c in stdin().keys() {
            match self.mode {
                Mode::Normal => {
                    match c.unwrap() {
                        Key::Char('q') => break,
                        Key::Char('/') => {
                            self.change_mode(Mode::Command);
                            self.command = "/".to_string();
                        },
                        Key::Char('a') => {
                            self.change_mode(Mode::Command);
                            self.command = ":add ".to_string();
                        },
                        Key::Char('g') => write!(self.stdout, "{}", termion::cursor::Goto(1, 1)).unwrap(),
                        Key::Char('G') => write!(self.stdout, "{}", termion::cursor::Goto(1, 5)).unwrap(),
                        Key::Char('j') | Key::Down =>self.move_cursor(1),
                        Key::Char('k') | Key::Up => self.move_cursor(-1),
                        // Key::Char(c) => println!("{}", c),
                        // Key::Alt(c) => println!("^{}", c),
                        // Key::Ctrl(c) => println!("*{}", c),
                        _ => {},
                    }
                }
                Mode::Command => {
                    match c.unwrap() {
                        Key::Esc => self.change_mode(Mode::Normal),
                        Key::Char('\n') => {
                            self.parse_command();
                            self.change_mode(Mode::Normal);
                        },
                        Key::Char(c) => {
                            let mut buf = [0u8; 4];
                            let c_str = c.encode_utf8(&mut buf);
                            self.command = self.command.clone() + c_str;
                        },
                        _ => {}
                    }

                    self.draw_command();
                }
            }

            self.stdout.flush().unwrap();
        }
    }

    fn move_cursor(&mut self, dy: i8) {
        let term_size = terminal_size().unwrap();
        let mut cursor = self.stdout.cursor_pos().unwrap();
        write!(self.stdout, "{} ", termion::cursor::Goto(0, cursor.1));
        
        cursor.1 = cursor.1.wrapping_add(dy as u16);
        write!(self.stdout, "{}>", termion::cursor::Goto(0, cursor.1));
    }

    fn change_mode(&mut self, mode: Mode) {
        match mode {
            Mode::Normal => {
                write!(self.stdout, "{}", termion::cursor::Goto(self.cursor_backup.0, self.cursor_backup.1)).unwrap();
            }
            Mode::Command => {
                self.cursor_backup = self.stdout.cursor_pos().unwrap();
                let term_size = terminal_size().unwrap();
                write!(self.stdout, "{}", termion::cursor::Goto(1, term_size.1)).unwrap();
            },
        }

        self.mode = mode;
    }

    fn parse_command(&mut self) {
        let c = self.command.chars().nth(0).unwrap();

        if c == '/' {
            println!("{}", &self.command[1..]);
        }
        else if c == ':' {
            // self.tasks.push(Task {
            //     done: false,
            //     name: self.command[5..].to_string(),
            //     project: "prj".to_string(),
            // });
            self.draw_tasks();
        }
    }

    fn draw_tasks(&mut self) {
        for (i, t) in self.tasks.iter().enumerate() {
            write!(self.stdout, "{}{}",
                termion::cursor::Goto(1, (i + 1) as u16),
                t.name
            ).unwrap();
        }
    }

    fn draw_command(&mut self) {
        let term_size = terminal_size().unwrap();

        match self.mode {
            Mode::Command => {
                write!(self.stdout, "{}{}{}",
                    termion::cursor::Goto(1, term_size.1),
                    termion::clear::CurrentLine,
                    self.command
                ).unwrap();
            }
            _ => {}
        }
    }
}

fn main() {
    // let mut app = App::new();
    // app.run();

    // let task = task::Task::from_string("x (A) 2022-03-04 2022-03-01 @c1 @c2 +p1 due:2022-03-06 \"hoge fuga piyo due\"");

    // let hoge = task::Task::new("due:2022-08-01");
    // println!("{:?}", hoge.due);
}