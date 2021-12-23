use std::{error::Error, 
    io, 
    thread,
    sync::mpsc, 
    time::Duration
};

use termion::{
    event::Key,
    raw::IntoRawMode,
    input::{MouseTerminal, TermRead},
    screen::AlternateScreen 
};

use tui::Terminal;
use tui::backend::TermionBackend;

use crate::todo::TodoList;
use crate::ui;

pub struct App {
    pub todo_list: TodoList,
    pub todo_lists: Vec<TodoList>
}

impl App {
    pub fn new() -> App {
        return App { 
            todo_list: TodoList::new(),
            todo_lists: Vec::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), io::Error> {
        let stdout = io::stdout().into_raw_mode()?;
        // let stdout = MouseTerminal::from(stdout);
        // let stdout = AlternateScreen::from(stdout);
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        self.todo_list.add_task("this is a task".to_string());
        self.todo_list.add_task("2 task 2".to_string());
        self.todo_list.add_task("task 3 3".to_string());
        self.todo_list.add_task("task 4 4 4 4".to_string());
        self.todo_list.add_task("5 task".to_string());

        self.todo_list.next();
        self.todo_list.remove();
        self.todo_list.next();

        terminal.clear();
        // loop {
        terminal.draw(|f| ui::draw(f, &mut self.todo_list))?;
        thread::sleep(Duration::from_secs(1));    
        // }
        
        Ok(())
    }
}
