use std::io;

use app::App;
use clap::Parser;
use crossterm::{event, execute, terminal};
use mpd_client::client::Client;
use ratatui::prelude::*;
use tokio::net::TcpStream;

mod app;
mod cli;
mod ui;

#[tokio::main]
async fn main() {
    let args = cli::Args::parse();

    // mpd connection
    let addr = format!("{}:{}", args.host, args.port);
    let connection = TcpStream::connect(addr).await.unwrap();
    let (client, _) = Client::connect(connection).await.unwrap();
    println!("connected successfully");

    // setup tui and app
    let mut terminal = setup(io::stdout()).unwrap();
    let mut app = App::init(client, args).unwrap();

    // run app (aka the actual functionality)
    match app.run(&mut terminal) {
        Ok(()) => {}
        Err(error) => println!("ERROR: {:?}", error),
    };

    // destroy tui (important to make sure the terminal is restored to normal)
    teardown(terminal).unwrap();
}

fn setup<W: io::Write>(mut writer: W) -> io::Result<Terminal<CrosstermBackend<W>>> {
    terminal::enable_raw_mode()?;
    execute!(
        &mut writer,
        terminal::EnterAlternateScreen,
        event::EnableMouseCapture
    )?;
    Terminal::new(CrosstermBackend::new(writer))
}
pub fn teardown<W: io::Write>(mut terminal: Terminal<CrosstermBackend<W>>) -> io::Result<()> {
    terminal::disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        terminal::LeaveAlternateScreen,
        event::DisableMouseCapture,
    )?;
    terminal.show_cursor()?;
    Ok(())
}
