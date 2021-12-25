use std::{error::Error, 
    io, 
    thread,
    sync::mpsc, 
    time::Duration
};

use termion::{
    event::Key,
    raw::IntoRawMode,
    input::{TermRead},
    screen::AlternateScreen,
    cursor::HideCursor,
    color,
    style
};

use tui::widgets::{ListState};

use tui::Terminal;
use tui::backend::TermionBackend;

use crate::todo::TodoList;
use crate::ui;

//add to separate events file -----------------------------
enum Event {
    Input(Key),
    Tick,
}

fn events(tick_rate: Duration) -> mpsc::Receiver<Event> {
    let (tx, rx) = mpsc::channel();
    let keys_tx = tx.clone();
    thread::spawn(move || {
        let stdin = io::stdin();
        for evt in stdin.keys() {
            if let Ok(key) = evt {
                if let Err(err) = keys_tx.send(Event::Input(key)) {
                    eprintln!("{}", err);
                    return;
                }
            }
        }
    });
    thread::spawn(move || loop {
        if let Err(err) = tx.send(Event::Tick) {
            eprintln!("{}", err);
            break;
        }
        thread::sleep(tick_rate);
    });
    rx
}

pub struct Options {
    pub state: ListState,
    // index: i64
}

impl Options {
    pub fn new() -> Options {
        return Options{
           state: ListState::default(),
            // index: 0;
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= 3 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    3
                } else {
                    i - 1 
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    // pub fn match_option(&mut self) {
    //     let i = match self.state.selected() {

    //     };
    // }
}

//---------------------------------------------------------

pub enum Mode {
    ListMode,
    OptionMode
}

pub struct App {
    pub todo_list: TodoList,
    pub todo_lists: Vec<TodoList>,
    pub options: Options,
    pub quit_state: bool,
    pub previous_mode: Mode,
    pub current_mode: Mode
}

impl App {
    pub fn new() -> App {
        return App { 
            todo_list: TodoList::new(),
            todo_lists: Vec::new(),
            options: Options::new(),
            quit_state: false,
            previous_mode: Mode::ListMode,
            current_mode: Mode::ListMode
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let stdout = io::stdout().into_raw_mode()?;
        // let stdout = MouseTerminal::from(stdout);
        let stdout = AlternateScreen::from(stdout);
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        terminal.clear()?;
        let events = events(Duration::from_millis(50));
        terminal.hide_cursor()?; //LINE OF GOD
        loop {
            terminal.draw(|f| ui::draw(f, self))?;
            match events.recv()? {
                Event::Input(key) => match key {
                    Key::Char(c) => {
                        self.translate_char_instruction(c);
                    }
                    Key::Esc => self.translate_escape(),
                    _ => {}
                }
                Event::Tick => {},
            }
            if self.quit_state == true {
                return Ok(())
            }
        }
        
        Ok(())
    }

    pub fn translate_escape(&mut self) {
        match self.current_mode {
            Mode::OptionMode => {
                self.current_mode = Mode::ListMode;
            }
            _ => self.current_mode = Mode::OptionMode
        }
    }

    pub fn translate_char_instruction(&mut self, c: char) {
        if c == 'q' {
            self.quit_state = true;
            return;
        }

        match self.current_mode {
            Mode::OptionMode => {
                if c == 'j' {
                    self.options.next();
                }
                if c == 'k' {
                    self.options.previous();
                }
                // if c == 'a' {
                //     self.options.select();
                // }
            },
            Mode::ListMode => {
                if c == 'j' {
                    self.todo_list.next();
                }
                if c == 'k' {
                    self.todo_list.previous();
                }
                if c == 'a' {
                    self.todo_list.add_task("default task".to_string());
                }
                if c == 'd' {
                    self.todo_list.remove();
                }
                if c == 'x' {
                    self.todo_list.mark_completeness();
                }
                if c == '1' || c == '2' || c == '3' {
                    self.todo_list.mark_with_tag(c);
                }
            }
        }
    }
}
