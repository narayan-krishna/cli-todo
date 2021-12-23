mod ui;
mod todo;
mod app;

use std::{error::Error, 
    io, 
    time::Duration
};

fn main() -> Result<(), io::Error> {
    let mut app = app::App::new();
    app.run();
    Ok(())
}


