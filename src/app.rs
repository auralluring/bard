use std::io;

use crossterm::event;
use crossterm::{execute, terminal};
use ratatui::prelude::*;

use crate::cli::Args;
use crate::ui::ui;

pub struct App<W: io::Write> {
    pub terminal: Terminal<CrosstermBackend<W>>,
    pub client: mpd_client::Client,
    pub args: Args,
}

impl<W: io::Write> App<W> {
    pub fn run(&mut self) -> Result<(), io::Error> {
        loop {
            self.terminal.draw(|f| ui(f, &app))?;
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
    pub fn init(mut writer: W, client: mpd_client::Client, args: Args) -> io::Result<Self> {
        terminal::enable_raw_mode()?;
        execute!(
            writer,
            terminal::EnterAlternateScreen,
            event::EnableMouseCapture
        )?;
        let backend = CrosstermBackend::new(writer);
        Ok(App {
            terminal: Terminal::new(backend)?,
            client,
            args,
        })
    }
    pub fn teardown(mut self) -> io::Result<()> {
        terminal::disable_raw_mode()?;
        execute!(
            self.terminal.backend_mut(),
            terminal::LeaveAlternateScreen,
            event::DisableMouseCapture,
        )?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}
