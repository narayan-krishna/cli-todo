use std::io;
use termion::raw::IntoRawMode;
use tui::Terminal;
use tui::backend::TermionBackend;

use crate::todo::TodoList;
use crate::ui;

pub struct App {
    pub todo_list: TodoList,
}

impl App {
    pub fn new() -> App {
        return App { todo_list: TodoList::new() }
    }

    pub fn run(&mut self) -> Result<(), io::Error> {
        let stdout = io::stdout().into_raw_mode()?;
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        self.todo_list.add_task("this is a task".to_string());
        self.todo_list.add_task("2 task 2".to_string());
        self.todo_list.add_task("task 3 3".to_string());
        self.todo_list.add_task("task 4 4 4 4".to_string());
        self.todo_list.add_task("5 task".to_string());

        terminal.clear();
        terminal.draw(|f| ui::draw(f, &mut self.todo_list))?;
        Ok(())
    }
}
