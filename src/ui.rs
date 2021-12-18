use tui::widgets::{Widget, Cell, ListItem, List, ListState, Block, Table, Row, Borders};
use tui::layout::{Layout, Constraint, Direction};
use tui::style::{Color, Modifier, Style};
use tui::backend::Backend;
use tui::text::{Span, Spans, Text};
use tui::Frame;

use crate::todo::TodoList;

//draw takes a frame, and a todo list object (contains a list, state)
pub fn draw<B: Backend>(f: &mut Frame<B>, todo: &mut TodoList) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(85),
                Constraint::Percentage(15),
                // Constraint::Percentage(10)
            ].as_ref()
        )
        .split(f.size());

    let block = Block::default()
            .title("Command")
            .borders(Borders::ALL);
    f.render_widget(block, chunks[1]);

    let items: Vec<ListItem> = todo
        .list
        .iter()
        .map(|p| {
            let item = Text::raw(p);
            ListItem::new(item)
        })
        .collect();

    let todo_list = List::new(items)
        .block(Block::default().title("List").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");

    f.render_stateful_widget(todo_list, chunks[0], &mut todo.state);
    // f.render_widget(list2, chunks[0]);

}

// pub fn draw_list() {

// }
