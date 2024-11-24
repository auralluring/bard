use ratatui::prelude::*;
use ratatui::widgets::*;

use crate::app::App;

pub fn ui<W: std::io::Write>(frame: &mut Frame, app: &App<W>) {
    let title = Paragraph::new(Text::styled(
        "Hello ratatui!",
        Style::default().fg(Color::Green),
    ));
    frame.render_widget(title, frame.area());
}
