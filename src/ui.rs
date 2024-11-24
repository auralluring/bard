use ratatui::prelude::*;
use ratatui::widgets::*;

pub fn ui(frame: &mut Frame) {
    let title = Paragraph::new(Text::styled(
        "Hello ratatui!",
        Style::default().fg(Color::Green),
    ));
    frame.render_widget(title, frame.area());
}
