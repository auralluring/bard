use std::io;

use crossterm::event;
use crossterm::{execute, terminal};
use ratatui::{layout::Layout, prelude::*};

use crate::ui::ui;

pub struct App<W: io::Write> {
    pub terminal: Terminal<CrosstermBackend<W>>,
    pub layout: Layout,
    client: mpd_client::Client,
}

impl<W: io::Write> App<W> {
    pub fn run(&mut self) -> Result<(), io::Error> {
        loop {
            self.terminal.draw(|f| ui(f))?;
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
    pub fn init(mut writer: W, client: mpd_client::Client) -> io::Result<Self> {
        terminal::enable_raw_mode()?;
        execute!(
            writer,
            terminal::EnterAlternateScreen,
            event::EnableMouseCapture
        )?;
        let backend = CrosstermBackend::new(writer);
        Ok(App {
            terminal: Terminal::new(backend)?,
            layout: Layout::default(),
            client,
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
