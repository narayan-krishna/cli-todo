use crate::todo::TodoList;
use tui::widgets::{Widget, Cell, ListItem, List, ListState, Block, Table, Row, Borders};
use tui::layout::{Layout, Constraint, Direction};
use tui::style::{Color, Modifier, Style};
use tui::backend::Backend;
use tui::Frame;

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

    // let block = Block::default()
    //         .title("Control")
    //         .borders(Borders::ALL);

    let items = [ListItem::new("1"), 
                 ListItem::new("25"), 
                 ListItem::new("32"), 
                 ListItem::new("4")];

    // let todo_items: vec<listitem> = todo
    //     .list
    //     .name

    let mut state = ListState::default();
    state.select(Some(1));

    // let list = List::new(items)
    //     .block(Block::default().title("to-do") .borders(Borders::ALL))
    //     .highlight_style(Style::default().add_modifier(Modifier::BOLD))
    //     .highlight_symbol(">>");

    let list2 = List::new(items)
        .block(Block::default().title("List").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");

    f.render_stateful_widget(list2, chunks[0], &mut state);
    // f.render_widget(list2, chunks[0]);

}