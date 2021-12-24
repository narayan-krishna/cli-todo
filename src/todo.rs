use tui::widgets::{ListState};
use chrono::{Local, DateTime};

//---------------------------------------------------------

pub struct TodoItem {
    pub name: String,
    pub completed: bool,
    pub date_started: DateTime<Local>,
    // pub date_finished: DateTime,
    pub date_last_modified: DateTime<Local>,
    pub description: String,
    pub priority_tag: String,
    // pub tags: Vec<String>
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
            description: "-- press <hotkey> to add description --".to_string(),
            priority_tag: "-".to_string(),
            // tags: Vec::new()
        };
    }

    pub fn get_name(&self) -> &String {
        return &self.name;
    }

    pub fn get_date_started_rfc(&self) -> String {
        return self.date_started.to_rfc2822();
    }

    pub fn get_date_last_modified_rfc(&self) -> String {
        return self.date_last_modified.to_rfc2822();
    }

    pub fn get_description(&self) -> &String {
        return &self.description;
    }

    pub fn add_description(&mut self, descript: String) {
        self.description =  descript;
        self.date_last_modified = Local::now() 
    }

    pub fn get_priority_tag(&self) -> &String {
        return &self.priority_tag;
    }

    pub fn add_priority_tag(&mut self, tag: String) {
        if tag == "1" || tag == "2" || tag == "3" {
            self.priority_tag = tag;
            self.date_last_modified = Local::now() 
        } else {
            println!("not valid priority tag");
        }
    }

    pub fn mark_completeness(&mut self) {
        self.completed = !self.completed;
    }

}

//---------------------------------------------------------

pub struct TodoList {
    pub name: String,
    pub uncompleted_list: Vec<TodoItem>,
    pub uncompleted_list_length: usize,
    pub completed_list: Vec<TodoItem>,
    pub state: ListState,
    pub completion_progress: f64,
    pub tasks_completed: usize,
    pub current_task_index: usize
    // index: i64
}

impl TodoList {
    pub fn new() -> TodoList {
        return TodoList{
            name: "New List".to_string(),
            uncompleted_list: Vec::new(), 
            uncompleted_list_length: 0,
            completed_list: Vec::new(), 
            state: ListState::default(),
            completion_progress: 0.0,
            tasks_completed: 0,
            current_task_index: 0
            // index: 0;
        }
    }

    pub fn next(&mut self) {
        if self.uncompleted_list_length != 0 {
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
    }

    pub fn previous(&mut self) {
        if self.uncompleted_list_length != 0 {
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
    }

    pub fn current_task_index(&self) -> usize {
        let i = match self.state.selected() {
            Some(i) => return i,
            None => return 0,
        };
    }

    pub fn get_task(&self, index: usize) -> &TodoItem {
        return &self.uncompleted_list[index];
    }

    pub fn add_task(&mut self, name: String) {
        let todo_item = TodoItem::new(name);
        if self.uncompleted_list_length == 0 {
            self.uncompleted_list.push(todo_item);
            self.state.select(Some(0));
        } else {
            let index = self.current_task_index() + 1;
            self.uncompleted_list.insert(index, todo_item);
            self.state.select(Some(index));
        }
        self.uncompleted_list_length = self.uncompleted_list_length + 1;
        self.update_completion_progress(true);
    }

    pub fn add_task_default(&mut self) {
        let todo_item = TodoItem::new("Default Task".to_string());
        self.uncompleted_list.push(todo_item);
        self.state.select(Some(self.uncompleted_list.len() - 1));
    }

    pub fn remove(&mut self) {
        if self.uncompleted_list_length != 0 {
            let i = match self.state.selected() {
                Some(i) => {
                    self.uncompleted_list.remove(i);
                    if self.uncompleted_list_length - 1 == i {
                        self.state.select(Some(0));
                    } else {
                        // self.next();
                    }
                } 
                None => println!("requires an element to remove!"),
            };
            self.uncompleted_list_length = self.uncompleted_list_length - 1;
        }
    }

    pub fn mark_completeness(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                let progress_direction = self.uncompleted_list[i].completed;
                self.update_completion_progress(progress_direction);
                self.uncompleted_list[i].mark_completeness();
            } 
            None => println!("requires an element to remove!"),
        };
    }

    pub fn mark_with_tag(&mut self, tag: char) {
        let string_tag = tag.to_string();
        let i = match self.state.selected() {
            Some(i) => {
                self.uncompleted_list[i].add_priority_tag(string_tag);
            } 
            None => println!("requires an element to remove!"),
        };
    }

    /*replace bool with enum*/
    pub fn update_completion_progress(&mut self, progress_direction: bool) {
        // true -> marked -> progress direction -> down
        if progress_direction == true {
            if self.tasks_completed != 0 {
                self.tasks_completed = self.tasks_completed - 1;
            }
        } else {
            self.tasks_completed = self.tasks_completed + 1;
        }
        let tasks_completed_f64: f64 = self.tasks_completed as f64;
        let length_f64: f64 = self.uncompleted_list_length as f64;
        self.completion_progress = tasks_completed_f64/length_f64;
    }

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
