use ratatui::{
    layout::Alignment,
    style::Style,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    frame.render_widget(
        Paragraph::new(format!(
            "This is a todo app.\n\
                Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
                Counter: {:?}",
            app.todos
        ))
        .block(
            Block::default()
                .title("genie todo")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default())
        .alignment(Alignment::Center),
        frame.size(),
    )
}
