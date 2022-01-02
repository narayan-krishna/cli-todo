use tui::widgets::{ListState};

pub struct Options {
    pub state: ListState,
    option_count: usize
    // pub options: Vec<String>
    // index: i64
}

impl Options {
    pub fn new() -> Options {
        return Options{
           state: ListState::default(),
           option_count: 4
        //    options: vec![];
            // index: 0;
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= 3 {
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
                    3
                } else {
                    i - 1 
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}
