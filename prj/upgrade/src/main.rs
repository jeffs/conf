mod app;
mod runner;
mod task;
mod ui;

use std::io;
use std::process::ExitCode;

use app::App;

#[tokio::main]
async fn main() -> io::Result<ExitCode> {
    let terminal = ratatui::init();
    let result = App::new().run(terminal).await;
    ratatui::restore();
    result
}
