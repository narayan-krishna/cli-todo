mod ui;
mod todo;

use std::io;
use termion::raw::IntoRawMode;
use tui::Terminal;
use tui::backend::TermionBackend;

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut todo_list = todo::TodoList::new();

    terminal.clear();
    terminal.draw(|f| ui::draw(f, &mut todo_list))?;
    Ok(())
}


