use tui::widgets::{Widget, Cell, ListItem, List, ListState, Block, Table, Row, Borders};
use tui::layout::{Layout, Constraint, Direction, Rect};
use tui::style::{Color, Modifier, Style};
use tui::backend::Backend;
use tui::text::{Span, Spans, Text};
use tui::Frame;

use crate::todo::TodoList;

pub fn draw<B: Backend>(f: &mut Frame<B>, todo: &mut TodoList) {
    draw_list_mode(f, todo);
}

pub fn draw_list_mode<B: Backend>(f: &mut Frame<B>, todo: &mut TodoList) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                // Constraint::Percentage(85),
                // Constraint::Percentage(15),
                Constraint::Min(0),
                Constraint::Length(3)
            ].as_ref()
        )
        .split(f.size());

    // draw_todo_list(f, chunks_vertical[0], todo);
    draw_top_panel(f, chunks[0], todo);
    draw_bot_panel(f, chunks[1]);
}


pub fn draw_top_panel<B: Backend>(f: &mut Frame<B>, chunk: Rect, todo: &mut TodoList) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        // .margin(1)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ].as_ref()
        )
        .split(chunk);

    draw_todo_list(f, chunks[0], todo);
    draw_command(f, chunks[1]);
}

pub fn draw_bot_panel<B: Backend>(f: &mut Frame<B>, chunk: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        // .margin(1)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ].as_ref()
        )
        .split(chunk);
    draw_command(f, chunks[0]);
    draw_command(f, chunks[1]);
}

pub fn draw_command<B: Backend>(f: &mut Frame<B>, chunk: Rect) {
    let block = Block::default()
            .title("Command")
            .borders(Borders::ALL);
    f.render_widget(block, chunk);
}

//draw takes a frame, and a todo list object (contains a list, state)
pub fn draw_todo_list<B: Backend>(f: &mut Frame<B>, chunk: Rect, todo: &mut TodoList) {
    let items: Vec<ListItem> = todo
        .list
        .iter()
        .map(|p| {
            let descript: &String = &p.name;
            let item = Text::raw(descript);
            ListItem::new(item)
        })
        .collect();

    let todo_list = List::new(items)
        .block(Block::default().title("List").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");

    f.render_stateful_widget(todo_list, chunk, &mut todo.state);
}

// pub fn draw_task_info<B: Backend>(f: &mut Frame<B>, chunk: Rect, todo: &mut TodoList) {

//     let task_info_items: Vec<ListItem> = todo
//         .list
//         .iter().
//         .map(|p| {
//             //let descript: &String = &p.name;
//             let item = Text::raw(tag + info);
//             ListItem::new(item)
//         }

//     let task_info = List::new(task_info_items)
//         .block(Block::default().title("Task Report").borders(Borders::ALL))
//         .style(Style::default().fg(Color::White))

//     f.render_stateful_widget(todo_list, chunk, &mut todo.state);
// }