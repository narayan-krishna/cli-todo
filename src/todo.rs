use tui::widgets::{ListState};
use chrono::{Local, Utc, DateTime};

//---------------------------------------------------------

pub struct TodoItem {
    pub name: String,
    pub completed: bool,
    pub date_started: DateTime<Local>,
    // pub date_finished: DateTime,
    pub date_last_modified: DateTime<Local>,
    pub description: String,
    pub tags: Vec<String>
    // pub finish_by: i64:
}

impl TodoItem {
    fn new(name: String) -> TodoItem {
        return TodoItem {
            name: name,
            completed: false,
            date_started: Local::now(),
            date_last_modified: Local::now(),
            // date_finished: ,
            description: "".to_string(),
            tags: Vec::new()
        };
    }

    // fn new(name: String, description: String) -> TodoItem {
    //     return TodoItem {
    //         name: name,
    //         completed: false,
    //         date_started: Local::now(),
    //         date_last_modified: Local::now(),
    //         // date_finished: ,
    //         description: description,
    //         tags: Vec::new()
    //     };
    // }

    pub fn get_name(&self) -> &String {
        return &self.name;
    }

    pub fn get_date_started_rfc(&self) -> String {
        return self.date_started.to_rfc2822();
    }

    pub fn get_date_last_modified_rfc(&self) -> String {
        return self.date_last_modified.to_rfc2822();
    }

    pub fn add_description(&mut self, descript: String) {
        self.description =  descript;
        self.date_last_modified = Local::now() 
    }

    pub fn add_tag(&mut self, tag: String) {
        self.tags.push(tag);
        self.date_last_modified = Local::now() 
    }
}

//---------------------------------------------------------

pub struct TodoList {
    pub name: String,
    pub uncompleted_list: Vec<TodoItem>,
    pub completed_list: Vec<TodoItem>,
    pub state: ListState,
    // index: i64
}

impl TodoList {
    pub fn new() -> TodoList {
        return TodoList{
            name: "New List".to_string(),
            uncompleted_list: Vec::new(), 
            completed_list: Vec::new(), 
            state: ListState::default(),
            // index: 0;
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.uncompleted_list.len() - 1 {
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
                   self.uncompleted_list.len() - 1 
                } else {
                    i - 1 
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn current_task_index(&self) -> usize {
        let i = match self.state.selected() {
            Some(i) => return i,
            None => return 0,
        };
    }

    pub fn add_task(&mut self, name: String) {
        let todo_item = TodoItem::new(name);
        self.uncompleted_list.push(todo_item);
        self.state.select(Some(self.uncompleted_list.len() - 1));
    }

    pub fn remove(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                self.uncompleted_list.remove(i);
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
