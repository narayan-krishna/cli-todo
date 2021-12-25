use tui::widgets::{Cell, ListItem, List, Paragraph, ListState, 
                   Gauge, Block, Clear, Borders, Wrap};
use tui::layout::{Alignment, Layout, Constraint, Direction, Rect};
use tui::style::{Color, Modifier, Style};
use tui::backend::Backend;
use tui::text::{Span, Spans, Text};
use tui::Frame;

use crate::todo::{TodoList, TodoItem};
use crate::app::{App, Mode};

// enum Mode {
//     ListMode,
//     OptionMode
// }
pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    match app.current_mode {
        Mode::OptionMode => {
            draw_options_mode(f, &mut app.options.state);
        },
        Mode::ListMode => {
            draw_list_mode(f, &mut app.todo_list);
        },
        _ => {}
    }
}

pub fn draw_options_mode<B: Backend>(f: &mut Frame<B>, options_state: &mut ListState) {
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

    let items = [ListItem::new("LOAD FILE"), ListItem::new("RECENT FILES"),
                 ListItem::new("KEYBINDS"), ListItem::new("CREDITS")];

    let option_list = List::new(items)
        .block(Block::default())
        .style(Style::default())
        .highlight_style(Style::default().add_modifier(Modifier::BOLD).bg(Color::Green));

    // f.render_widget(Clear, rect);
    f.render_stateful_widget(option_list, rect, options_state);
}

pub fn draw_list_mode<B: Backend>(f: &mut Frame<B>, todo: &mut TodoList) {
    let mut chunks = Layout::default()
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

    let offset = 1; 
    chunks[1].y = chunks[1].y - offset;
    chunks[1].height = chunks[1].height + offset - 1;

    // chunks[1].width = chunks[1].width - (offset*2);
    // chunks[1].x = chunks[1].x + offset;

    draw_bot_panel(f, chunks[1], todo);
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

    chunks[0].height = chunks[0].height - (offset*2) - 1;
    chunks[0].y = chunks[0].y + offset;

    // chunks[1].width = chunks[1].width - (offset*2);
    // chunks[1].x = chunks[1].x + offset;

    chunks[1].height = chunks[1].height - (offset*2);
    chunks[1].y = chunks[1].y + offset;

    // chunks[0].width = chunks[0].width - 1;

    draw_todo_list(f, chunks[0], todo);
    draw_todo_info(f, chunks[1], todo);
}


//draw takes a frame, and a todo list object (contains a list, state)
pub fn draw_todo_list<B: Backend>(f: &mut Frame<B>, chunk: Rect, todo: &mut TodoList) {
    let items: Vec<ListItem> = todo
        .uncompleted_list
        .iter()
        .map(|p| {
            let descript: &String = &p.name;
            let prio_tag: &String = p.get_priority_tag();
            let completion_tag: String;
            if p.completed {
                completion_tag = "x".to_string();
            } else {
                completion_tag = " ".to_string();
            }
            let item = Text::raw("|".to_owned() + prio_tag + "|" 
                + &completion_tag + "| " + descript);
            ListItem::new(item)
        })
        .collect();

    let todo_list = List::new(items)
        .block(Block::default()
        .title(Span::raw(&todo.name))
        .title_alignment(Alignment::Center))
        // .borders(Borders::ALL))
        .style(Style::default())
        .highlight_style(Style::default().add_modifier(Modifier::BOLD).bg(Color::Green));
        // .highlight_symbol(">> ");

    f.render_stateful_widget(todo_list, chunk, &mut todo.state);
}

pub fn draw_todo_info<B: Backend>(f: &mut Frame<B>, chunk: Rect, todo: &mut TodoList) {


    //change to lambda function
    let mut todo_descript_lines = Vec::new();
    if todo.uncompleted_list_length == 0 {
        todo_descript_lines.push(Spans::from("task lisk empty"));
    }  else {
        let curr_index = todo.current_task_index();
        let curr_item: &TodoItem = &todo.uncompleted_list[curr_index];

        todo_descript_lines.push(Spans::from(Span::raw("TASK: ".to_string() + &curr_item.get_name())));
        todo_descript_lines.push(Spans::from(Span::raw("TASK PRIORITY LEVEL: ".to_string() 
                            + &curr_item.get_priority_tag())));
        todo_descript_lines.push(Spans::from(""));
        todo_descript_lines.push(Spans::from(Span::raw("Date created: ".to_string() 
                            + &curr_item.get_date_started_rfc())));
        todo_descript_lines.push(Spans::from(Span::raw("Date last modified: ".to_string() 
                            + &curr_item.get_date_last_modified_rfc())));
        todo_descript_lines.push(Spans::from(""));
        todo_descript_lines.push(Spans::from("Description: ".to_string() + &curr_item.get_description()));
    }

    let todo_descript_paragraph = Paragraph::new(todo_descript_lines)
        .block(Block::default()
        .title("Task Report")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL))
        .style(Style::default())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    f.render_widget(todo_descript_paragraph, chunk);
}

pub fn draw_bot_panel<B: Backend>(f: &mut Frame<B>, chunk: Rect, todo: &mut TodoList) {
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
    // draw_corner_block(f, chunk);
    // draw_date_time(f, chunks[1]);
    draw_gauge_line(f, chunk, todo);
}

pub fn draw_corner_block<B: Backend>(f: &mut Frame<B>, mut chunk: Rect) {
    // let mut rect = f.size();
    // rect.x = rect.width - 5;
    let height_boost = 1; 
    chunk.y = chunk.y - height_boost;
    // rect.width = 5;
    chunk.height = chunk.height + height_boost;

    let corner_block = Block::default()
        .border_style(Style::default().fg(Color::White))
        .style(Style::default().bg(Color::Red));
    f.render_widget(corner_block, chunk);
}



pub fn draw_command<B: Backend>(f: &mut Frame<B>, chunk: Rect) {
    let block = Block::default()
            .title("Command")
            .borders(Borders::ALL);
    f.render_widget(block, chunk);
}

pub fn draw_gauge_line<B: Backend>(f: &mut Frame<B>, mut chunk: Rect, todo: &mut TodoList) {
    let gauge = Gauge::default()
        .block(Block::default().title("Completion Gauge").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Yellow))
        .ratio(todo.completion_progress);

    f.render_widget(gauge, chunk);
}

// }