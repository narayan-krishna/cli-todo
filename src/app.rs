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
//---------------------------------------------------------

pub struct App {
    pub todo_list: TodoList,
    pub todo_lists: Vec<TodoList>,
    quit: bool,
}

impl App {
    pub fn new() -> App {
        return App { 
            todo_list: TodoList::new(),
            todo_lists: Vec::new(),
            quit: false,
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let stdout = io::stdout().into_raw_mode()?;
        // let stdout = MouseTerminal::from(stdout);
        let stdout = AlternateScreen::from(stdout);
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;


        self.todo_list.add_task("this is a task".to_string());
        self.todo_list.add_task("2 task 2".to_string());
        self.todo_list.add_task("task 3 3".to_string());
        self.todo_list.add_task("task 4 4 4 4".to_string());
        self.todo_list.add_task("5 task".to_string());

        self.todo_list.uncompleted_list[1]
            .add_description("this is a description for blah!".to_string());
        self.todo_list.uncompleted_list[2]
            .add_description("ANOTHA ONE THIS HERE DESCRIPI".to_string());
        self.todo_list.uncompleted_list[4]
            .add_description("3RQFIUE BACK IN DA HOOD".to_string());


        terminal.clear();
        let events = events(Duration::from_millis(50));
        loop {
            //register event
            // terminal.show_cursor();
            terminal.show_cursor();
            terminal.draw(|f| ui::draw(f, &mut self.todo_list))?;
            match events.recv()? {
                Event::Input(key) => match key {
                    Key::Char(c) => {
                        if c == 'q' {
                            self.quit = true;
                        }
                        if c == 'j' {
                            self.down();
                        }
                        if c == 'k' {
                            self.up();
                        }
                    }
                    _ => {}
                }
                Event::Tick => {},
            }
            if self.quit == true {
                return Ok(())
            }
        }
        
        Ok(())
    }

    fn down(&mut self) {
        self.todo_list.next();
    }

    fn up(&mut self) {
        self.todo_list.previous();
    }
}
