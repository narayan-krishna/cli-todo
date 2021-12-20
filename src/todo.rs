use tui::widgets::{ListState};

//---------------------------------------------------------

pub struct TodoItem {
    pub name: String,
    pub completed: char,
    // pub date_started: 
    // pub date_finished:
    // pub finish_by:
}

impl TodoItem {
    fn new(name: String) -> TodoItem {
        return TodoItem {
            name: name,
            completed: ' ',

        };
    }

    // pub fn set_name(&mut self) {

    // }
}

//---------------------------------------------------------

pub struct TodoList {
    // pub list: Vec<String>,
    pub list: Vec<TodoItem>,
    pub state: ListState,
    // index: i64
}

impl TodoList {
    pub fn new() -> TodoList {
        return TodoList{
            list: Vec::new(), 
            state: ListState::default(),
            // index: 0;
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.list.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                   self.list.len() - 1 
                } else {
                    i - 1 
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    // pub fn add_task(&mut self, name: String) {
    //     self.list.push(name);
    //     self.state.select(Some(self.list.len() - 1));
    // }

    pub fn add_task(&mut self, name: String) {
        let todo_item = TodoItem::new(name);
        self.list.push(todo_item);
        self.state.select(Some(self.list.len() - 1));
    }

    pub fn remove(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                self.list.remove(i);
            } 
            None => println!("requires an element to remove!"),
        };
    }

    // pub fn mark_as_complete(&mut self, index: usize) {
    //     if x > 0 && x < list.size() { 
    //     }
    //     self.list[index].completed = 'x';
    // }

    // pub fn mark_as_uncomplete(&mut self, index: usize) {
    //     // if x > 0 && x < list.size()
    //     self.list[index].completed = ' ';
    // }


    // fn print(&self) {
    //     for (index, item) in self.list.iter().enumerate() {
    //         println!("{} -- [{}] -- {}", index, item.completed, item.name);
    //     }
    // }
    // pub fn set_due_date(&mut self, date: Date) {

    // }
}

//---------------------------------------------------------

// use std::env;


// enum Command {
//     Get,
//     Add(String),
//     Done(usize),
//     Unmark(usize),
//     Remove(usize)
// }

// fn main() {

//     let mut todo_list = TodoList::new();
//     todo_list.add_task("task 1".to_string());
//     todo_list.add_task("task 2".to_string());
//     todo_list.add_task("task 3".to_string());
//     todo_list.add_task("task 4".to_string());
//     todo_list.mark_as_complete(3);
//     todo_list.mark_as_complete(1);
//     todo_list.mark_as_uncomplete(3);
//     todo_list.print();
    
//     print!("--------------------\n");

//     let args: Vec<String> = env::args().collect();
//     // print!("{}\n", args.len());

//     let command = match args[1].as_str() {
//        "get" => Command::Get,
//        "add" => Command::Add(args[2].clone()),
//        "done" => Command::Done(args[2].parse().expect("error for integer convert")),
//        "unmark" => Command::Unmark(args[2].parse().expect("error for integer convert")),
//        "remove" => Command::Remove(args[2].parse().expect("error for integer convert")),
//        _ => panic!("must provide a command"),
//     };


//     match command {
//         Command::Get => todo_list.print(),
//         Command::Add(task) => {
//             todo_list.add_task(task);
//         },
//         Command::Done(index) => todo_list.mark_as_complete(index),
//         Command::Unmark(index) => todo_list.mark_as_uncomplete(index),
//         Command::Remove(index) => todo_list.remove(index)
//     }

//     todo_list.print();
// }
