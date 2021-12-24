use tui::widgets::{Widget, Cell, ListItem, List, Paragraph,
                   ListState, Block, Table, Row, Borders, Wrap};
use tui::layout::{Alignment, Layout, Constraint, Direction, Rect};
use tui::style::{Color, Modifier, Style};
use tui::backend::Backend;
use tui::text::{Span, Spans, Text};
use tui::Frame;

use crate::todo::{TodoList, TodoItem};

pub fn draw<B: Backend>(f: &mut Frame<B>, todo: &mut TodoList, options: &bool) {
    if options == &true {
        draw_options_window(f);
    } else {
        draw_list_mode(f, todo);
    }
}

pub fn draw_corner_block<B: Backend>(f: &mut Frame<B>, rect: Rect) {
    // let mut rect = f.size();
    // rect.x = rect.width - 5;
    // rect.y = rect.height - 5;
    // rect.width = 5;
    // rect.height = 5;

    let corner_block = Block::default()
        .border_style(Style::default().fg(Color::White))
        .style(Style::default().bg(Color::Black));
    f.render_widget(corner_block, rect);
}

pub fn draw_options_window<B: Backend>(f: &mut Frame<B>) {
    // let options: Vec<String> = todo
    //     .uncompleted_list
    //     .iter()
    //     .map(|p| {
    //         let descript: &String = &p.name;
    //         let prio_tag: &String = p.get_priority_tag();
    //         let option = Text::raw("|".to_owned() + prio_tag + "| " + descript);
    //         ListItem::new(option)
    //     })
    //     .collect();
    // let margin = 
    let frame_top_length = f.size().width;
    let offset = 15;
    let rect = Rect::new(frame_top_length/4 + offset, 5, (frame_top_length/2) - (offset*2), 20);

    let items = [ListItem::new("OPTION 1"), ListItem::new("OPTION 2"),
                 ListItem::new("OPTION 3"), ListItem::new("OPTION 4")];

    let todo_list = List::new(items)
        .block(Block::default())
        // .title(Span::raw("Options"))
        // .title_alignment(Alignment::Center)
        // .borders(Borders::ALL))
        .style(Style::default().fg(Color::White).bg(Color::Black));
        // .highlight_style(Style::default().add_modifier(Modifier::BOLD).bg(Color::Green));
        // .highlight_symbol(">> ");

    f.render_widget(todo_list, rect);

}

pub fn draw_list_mode<B: Backend>(f: &mut Frame<B>, todo: &mut TodoList) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        // .margin(2)// -- LINE OF DEATH
        .constraints(
            [
                // Constraint::Percentage(85),
                // Constraint::Percentage(15),
                Constraint::Min(0),
                Constraint::Length(3),
                // Constraint::Max(3)
            ].as_ref()
        )
        .split(f.size());
        // .style(Style::default().fg(Color::Black));

    // draw_todo_list(f, chunks_vertical[0], todo);
    draw_top_panel(f, chunks[0], todo);
    draw_bot_panel(f, chunks[1]);
}


pub fn draw_top_panel<B: Backend>(f: &mut Frame<B>, chunk: Rect, todo: &mut TodoList) {
    let mut chunks = Layout::default()
        .direction(Direction::Horizontal)
        // .margin(1)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ].as_ref()
        )
        .split(chunk);

    let offset = 1;

    chunks[0].width = chunks[0].width - (offset*2);
    chunks[0].x = chunks[0].x + offset;

    chunks[0].height = chunks[0].height - (offset*2);
    chunks[0].y = chunks[0].y + offset;

    chunks[1].width = chunks[1].width - (offset*2);
    chunks[1].x = chunks[1].x + offset;

    chunks[1].height = chunks[1].height - (offset*2);
    chunks[1].y = chunks[1].y + offset;

    // chunks[0].width = chunks[0].width - 1;

    draw_todo_list(f, chunks[0], todo);
    draw_todo_info(f, chunks[1], todo);
}

pub fn draw_bot_panel<B: Backend>(f: &mut Frame<B>, chunk: Rect) {
    // let mut chunks = Layout::default()
    //     .direction(Direction::Horizontal)
    //     // .margin(1)
    //     .constraints(
    //         [
    //             Constraint::Percentage(75),
    //             Constraint::Percentage(25),
    //         ].as_ref()
    //     )
    //     .split(chunk);

    // let offset = 1;

    // draw_command(f, chunks[0]);
    // draw_corner_block(f, chunks[1]);
    draw_corner_block(f, chunk);
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
            let prio_tag: &String = p.get_priority_tag();
            let item = Text::raw("|".to_owned() + prio_tag + "| " + descript);
            ListItem::new(item)
        })
        .collect();

    let todo_list = List::new(items)
        .block(Block::default()
        .title(Span::raw(&todo.name))
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL))
        .style(Style::default())
        .highlight_style(Style::default().add_modifier(Modifier::BOLD).bg(Color::Blue));
        // .highlight_symbol(">> ");

    f.render_stateful_widget(todo_list, chunk, &mut todo.state);
}

pub fn draw_todo_info<B: Backend>(f: &mut Frame<B>, chunk: Rect, todo: &mut TodoList) {

    let curr_index = todo.current_task_index();
    let curr_item: &TodoItem = &todo.uncompleted_list[curr_index];

    let todo_descript_lines = vec![
        Spans::from(Span::raw("TASK: ".to_string() + &curr_item.get_name())),
        Spans::from(Span::raw("TASK PRIORITY LEVEL: ".to_string() 
                            + &curr_item.get_priority_tag())),
        Spans::from(""),
        Spans::from(Span::raw("Date created: ".to_string() 
                              + &curr_item.get_date_started_rfc())),
        Spans::from(Span::raw("Date last modified: ".to_string() 
                              + &curr_item.get_date_last_modified_rfc())),
        Spans::from(""),
        Spans::from("Description: ".to_string() + &curr_item.get_description()),
    ];

    let todo_descript_paragraph = Paragraph::new(todo_descript_lines)
        .block(Block::default()
        .title("Task Report")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL))
        .style(Style::default().bg(Color::Black))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    f.render_widget(todo_descript_paragraph, chunk);
}

// pub fn draw_date_time<B: Backend>(f: &mut Frame<B>, chunk: Rect, todo: &mut TodoList) {

// }