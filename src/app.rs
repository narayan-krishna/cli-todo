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

enum Mode {
    ListMode,
    OptionMode
}

//---------------------------------------------------------

pub struct App {
    pub todo_list: TodoList,
    pub todo_lists: Vec<TodoList>,
    pub options: Options,
    quit_state: bool,
    show_options: bool,
    mode: Mode
}

impl App {
    pub fn new() -> App {
        return App { 
            todo_list: TodoList::new(),
            todo_lists: Vec::new(),
            options: Options::new(),
            show_options: false,
            quit_state: false,
            mode: Mode::ListMode
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let stdout = io::stdout().into_raw_mode()?;
        // let stdout = MouseTerminal::from(stdout);
        let stdout = AlternateScreen::from(stdout);
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;


        // self.todo_list.add_task("this is a task".to_string());
        // self.todo_list.add_task("2 task 2".to_string());
        // self.todo_list.add_task("task 3 3".to_string());
        // self.todo_list.add_task("task 4 4 4 4".to_string());
        // self.todo_list.add_task("5 task".to_string());

        // self.todo_list.uncompleted_list[1]
        //     .add_description("this is a description for blah!".to_string());
        // self.todo_list.uncompleted_list[2]
        //     .add_description("ANOTHA ONE THIS HERE DESCRIPI".to_string());
        // self.todo_list.uncompleted_list[4]
        //     .add_description("3RQFIUE BACK IN DA HOODDDDDDDDDDDDDDDDDDDDDDDDDDDDD".to_string());
            
        // self.todo_list.uncompleted_list[0]
        //     .add_priority_tag("1".to_string());
        // self.todo_list.uncompleted_list[2]
        //     .add_priority_tag("3".to_string());
        // self.todo_list.uncompleted_list[3]
        //     .add_priority_tag("2".to_string());

        terminal.clear()?;
        let events = events(Duration::from_millis(50));
        terminal.hide_cursor()?; //LINE OF GOD
        loop {
            //register event
            // terminal.show_cursor();
            // terminal.set_cursor(100,100);
            terminal.draw(|f| ui::draw(f, &mut self.todo_list, &self.show_options, &mut self.options.state))?;
            match events.recv()? {
                Event::Input(key) => match key {
                    Key::Char(c) => {
                        if c == 'q' {
                            self.quit_state = true;
                        }
                        if c == 'o' {
                            self.show_options = !self.show_options;
                        }
                        if c == 'j' {
                            self.down();
                        }
                        if c == 'k' {
                            self.up();
                        }
                        if c == 'a' {
                            self.add();
                        }
                        if c == 'd' {
                            self.remove();
                        }
                        if c == 'v' {
                            self.mark();
                        }
                        if c == '1' || c == '2' || c == '3' {
                            self.nums(c);
                        }
                    }
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

    fn down(&mut self) {
        if self.show_options == true {
            self.options.next();
        } else {
            self.todo_list.next();
        }
    }

    fn up(&mut self) {
        if self.show_options == true {
            self.options.previous();
        } else {
            self.todo_list.previous();
        }
    }

    fn add(&mut self) {
        if self.show_options == true {
            {}
        } else {
            self.todo_list.add_task("default task".to_string());
        }
    }

    fn remove(&mut self) {
        if self.show_options == true {
            {}
        } else {
            self.todo_list.remove();
        }
    }

    fn mark(&mut self) {
        if self.show_options == true {
            {}
        } else {
            self.todo_list.mark_completeness();
        }
    }

    fn nums(&mut self, num: char) {
        if self.show_options == true {
            {}
        } else {
            self.todo_list.mark_with_tag(num);
        }
    }
}
