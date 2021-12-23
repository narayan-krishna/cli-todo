use tui::widgets::{Widget, Cell, ListItem, List, Paragraph,
                   ListState, Block, Table, Row, Borders};
use tui::layout::{Alignment, Layout, Constraint, Direction, Rect};
use tui::style::{Color, Modifier, Style};
use tui::backend::Backend;
use tui::text::{Span, Spans, Text};
use tui::Frame;

use crate::todo::{TodoList, TodoItem};

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
    draw_todo_info(f, chunks[1], todo);
}

pub fn draw_bot_panel<B: Backend>(f: &mut Frame<B>, chunk: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        // .margin(1)
        .constraints(
            [
                Constraint::Percentage(75),
                Constraint::Percentage(25),
            ].as_ref()
        )
        .split(chunk);
    draw_command(f, chunks[0]);
    draw_command(f, chunks[1]);
    // draw_date_time(f, chunks[1]);
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
        .uncompleted_list
        .iter()
        .map(|p| {
            let descript: &String = &p.name;
            let item = Text::raw(descript);
            ListItem::new(item)
        })
        .collect();

    let todo_list = List::new(items)
        .block(Block::default()
        .title(Span::raw(&todo.name))
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");

    f.render_stateful_widget(todo_list, chunk, &mut todo.state);
}

pub fn draw_todo_info<B: Backend>(f: &mut Frame<B>, chunk: Rect, todo: &mut TodoList) {

    let curr_index = todo.current_task_index();
    let curr_item: &TodoItem = &todo.uncompleted_list[curr_index];

    let todo_descript_lines = vec![
        Spans::from(Span::raw("TASK: ".to_string() + &curr_item.get_name())),
        Spans::from(""),
        Spans::from(Span::raw("Date created: ".to_string() 
                              + &curr_item.get_date_started_rfc())),
        Spans::from(Span::raw("Date last modified: ".to_string() 
                              + &curr_item.get_date_last_modified_rfc())),
        Spans::from("Description: ".to_string()),
    ];

    let todo_descript_paragraph = Paragraph::new(todo_descript_lines)
        .block(Block::default()
        .title("Task Report")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL))
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .alignment(Alignment::Left);
        // .wrap(Wrap { trim: true });

    f.render_widget(todo_descript_paragraph, chunk);
}

// pub fn draw_date_time<B: Backend>(f: &mut Frame<B>, chunk: Rect, todo: &mut TodoList) {

// }