use ratatui::{layout::{Constraint, Direction, Layout}, style::{Color, Style}, text::Span, widgets::{List, ListItem}, Frame};

use crate::app::App;

pub fn ui(frame: &mut Frame, app: &App) {
    match app.current_screen {
        crate::app::AppScreen::Menu => render_menu(frame, app),
        crate::app::AppScreen::Sort => todo!(),
    }
    
}

fn render_menu(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .split(frame.area());

    let list = List::new(
        app.algorithms.iter().map(|alg|
            ListItem::new(
                Span::styled(alg.0, Style::default().fg(Color::Red))
            )
        )
        .collect::<Vec<ListItem>>()
    );

    frame.render_widget(list, chunks[1]);
}