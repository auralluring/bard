use std::io;

use crossterm::{event, execute, terminal};
use mpd_client::client::Client;
use mpd_client::commands;
use mpd_client::protocol::response::Error;
use mpd_client::tag::Tag;
use ratatui::prelude::*;
use ratatui::widgets::*;
use tokio::net::TcpStream;

mod cli;
use clap::Parser;

#[tokio::main]
async fn main() {
    let args = cli::Args::parse();

    // mpd connection
    let addr = format!("{}:{}", args.host, args.port);
    let connection = TcpStream::connect(addr).await.unwrap();
    let (client, _) = Client::connect(connection).await.unwrap();
    println!("connected successfully");

    // enter tui
    let mut terminal = enter_tui(io::stdout()).unwrap();
    match run(&mut terminal) {
        Ok(()) => {}
        Err(error) => println!("ERROR: {:?}", error),
    };

    exit_tui(terminal).unwrap();
}

fn enter_tui<W: io::Write>(mut writer: W) -> io::Result<Terminal<CrosstermBackend<W>>> {
    terminal::enable_raw_mode()?;
    execute!(
        &mut writer,
        terminal::EnterAlternateScreen,
        event::EnableMouseCapture
    )?;
    // .unwrap();
    let backend = CrosstermBackend::new(writer);
    Ok(Terminal::new(backend).unwrap())
}

fn exit_tui<W: io::Write>(mut terminal: Terminal<CrosstermBackend<W>>) -> io::Result<()> {
    terminal::disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        terminal::LeaveAlternateScreen,
        event::DisableMouseCapture,
    )?;
    terminal.show_cursor()?;
    Ok(())
}

fn run<W: io::Write>(terminal: &mut Terminal<CrosstermBackend<W>>) -> Result<(), io::Error> {
    loop {
        terminal.draw(|f| ui(f))?;
        if let event::Event::Key(key) = event::read()? {
            if key.kind != event::KeyEventKind::Press {
                continue;
            }
            match key.code {
                event::KeyCode::Char('q') => return Ok(()),
                _ => {}
            }
        }
    }
}
fn ui(frame: &mut Frame) {
    let title = Paragraph::new(Text::styled(
        "Hello ratatui!",
        Style::default().fg(Color::Green),
    ));
    frame.render_widget(title, frame.area());
}
