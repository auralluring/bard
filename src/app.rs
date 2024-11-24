use std::io;

use crossterm::event;
use crossterm::{execute, terminal};
use ratatui::prelude::*;

use crate::cli::Args;
use crate::ui::ui;

pub struct App {
    pub client: mpd_client::Client,
    pub args: Args,
}

impl App {
    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<(), io::Error> {
        loop {
            terminal.draw(|f| ui(f, &self))?;
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
    pub fn init(client: mpd_client::Client, args: Args) -> io::Result<Self> {
        Ok(App { client, args })
    }
}
