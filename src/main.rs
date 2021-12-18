mod ui;
mod todo;
mod app;

use std::io;

fn main() -> Result<(), io::Error> {
    let mut app = app::App::new();
    app.run();
    Ok(())
}


