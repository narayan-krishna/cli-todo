// use std::{error::Error, 
//     io, 
//     thread,
//     sync::mpsc, 
//     time::{Duration, Instant}
// };

// use crossterm::{
//     event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
//     execute,
//     terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
//     cursor::{DisableBlinking, EnableBlinking, Hide}
// };


// use tui::backend::CrosstermBackend;
// use tui::Terminal;

// use crate::todo::TodoList;
// use crate::ui;

// pub struct App {
//     pub todo_list: TodoList,
//     pub todo_lists: Vec<TodoList>,
//     quit: bool,
// }

// impl App {
//     pub fn new() -> App {
//         return App { 
//             todo_list: TodoList::new(),
//             todo_lists: Vec::new(),
//             quit: false,
//         }
//     }

//     pub fn run(&mut self, tick_rate: Duration) -> Result<(), Box<dyn Error>> {
//         enable_raw_mode()?;
//         let mut stdout = io::stdout();
//         execute!(stdout, EnterAlternateScreen, DisableMouseCapture)?;
//         let backend = CrosstermBackend::new(stdout);
//         let mut terminal = Terminal::new(backend)?;


//         self.todo_list.add_task("this is a task".to_string());
//         self.todo_list.add_task("2 task 2".to_string());
//         self.todo_list.add_task("task 3 3".to_string());
//         self.todo_list.add_task("task 4 4 4 4".to_string());
//         self.todo_list.add_task("5 task".to_string());

//         // self.todo_list


//         let mut last_tick = Instant::now();
//         let hide = crossterm::cursor::Hide;
//         loop {
//             //register event
//             terminal.draw(|f| ui::draw(f, &mut self.todo_list))?;
//             let timeout = tick_rate
//                 .checked_sub(last_tick.elapsed())
//                 .unwrap_or_else(|| Duration::from_secs(0));
//             if crossterm::event::poll(timeout)? {
//                 if let Event::Key(key) = event::read()? {
//                     match key.code {
//                         KeyCode::Char(c) => {
//                             if c == 'q' {
//                                 self.quit = true;
//                             }
//                             if c == 'j' {
//                                 self.todo_list.next();
//                             }
//                             if c == 'k' {
//                                 self.todo_list.previous();
//                             }
//                         }
//                         _ => {}
//                     }
//                 }
//             }
//             if last_tick.elapsed() >= tick_rate {
//                 // app.on_tick();
//                 last_tick = Instant::now();
//             }
//             if self.quit == true {
//                 return Ok(());
//             }            
//         }
//     }



// }
