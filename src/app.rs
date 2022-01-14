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

use crate::options::Options;
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

//---------------------------------------------------------

pub enum Mode {
    ListMode,
    OptionMode,
    InputMode
}

pub enum InputAction {
    ChangeName,
    ChangeDescription
}

pub struct App {
    pub todo_list: TodoList,
    pub todo_lists: Vec<TodoList>,
    pub options: Options,
    pub quit_state: bool,
    pub input: String,
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
            input: String::new(),
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
            if let Event::Input(key) = events.recv()? {
                match self.current_mode {
                    Mode::OptionMode => match key {
                        Key::Char('q') => self.quit_state = true,
                        Key::Char('j') => self.options.next(),
                        Key::Char('k') => self.options.previous(),
                        Key::Char('o') => self.current_mode = Mode::ListMode,
                        _ => {}
                    },
                    Mode::ListMode => match key {
                        Key::Char('q') => self.quit_state = true,
                        Key::Char('j') => self.todo_list.next(),
                        Key::Char('k') => self.todo_list.previous(),
                        // Key::Char('a') => self.todo_list
                        //                     //i want to get input
                        //                       .add_task("default task".to_string()),
                        Key::Char('a') => {
                            self.todo_list.add_task(self.input.to_string());
                            self.current_mode = Mode::InputMode;
                        }
                        Key::Char('d') => self.todo_list.remove(),
                        Key::Char('x') => self.todo_list.mark_completeness(),
                        Key::Char('o') => self.current_mode = Mode::OptionMode,
                        Key::Char('i') => self.current_mode = Mode::InputMode,
                        _ => {}
                        // if c == '1' || c == '2' || c == '3' {
                        //     self.todo_list.mark_with_tag(c);
                        // }
                    },
                    Mode::InputMode => match key {
                        Key::Char('\n') => {
                            self.current_mode = Mode::ListMode;
                            self.todo_list.change_current_task_name(self.input.to_string());
                            self.input.clear();
                        }
                        Key::Esc => { 
                            self.current_mode = Mode::ListMode;
                            self.input.clear();
                        }
                        Key::Char(c) => { self.input.push(c); }
                        Key::Backspace => { self.input.pop(); }
                        _ => {}
                    },
                }
            }
            if self.quit_state == true {
                return Ok(())
            }
        }
        
        Ok(())
    }
}
