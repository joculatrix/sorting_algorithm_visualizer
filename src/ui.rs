use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Span,
    widgets::{Bar, BarChart, BarGroup, Block, Borders, List, ListItem, Paragraph},
    Frame
};

use crate::app::{App, AppScreen};

pub fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    // TITLE

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new("Sorting Algorithm Visualizer")
        .alignment(Alignment::Center)
        .block(title_block);

    frame.render_widget(title, chunks[0]);

    // CONTENT

    let content_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(20),
            Constraint::Min(100),
        ])
        .split(chunks[1]);

        render_menu(frame, content_area[0], app);
        render_sort(frame, content_area[1], app);

    // FOOTER

    let footer_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let footer = Paragraph::new(
        match app.current_screen {
            AppScreen::Menu => "<Enter> to confirm, <Esc> to quit",
            AppScreen::Sort => "<Esc> to return",
        }
    ).alignment(Alignment::Center).block(footer_block);

    frame.render_widget(footer, chunks[2]);
    
}

fn render_menu(frame: &mut Frame, area: Rect, app: &App) {
    let mut list_items = vec![];

    let (item_style, selected_style) = match app.current_screen {
        AppScreen::Menu => (
            Style::default().fg(Color::Red),
            Style::default().bg(Color::Red).fg(Color::Black),
        ),
        AppScreen::Sort => (
            Style::default().fg(Color::DarkGray),
            Style::default().bg(Color::DarkGray).fg(Color::Black),
        )
    };

    for i in 0..app.algorithms.len() {
        list_items.push(
            ListItem::new(
                Span::styled(
                    app.algorithms[i].name,
                    if i == app.selected {
                        selected_style
                    } else {
                        item_style
                    }
                )
            )
        );
    }

    let list = List::new(list_items);

    frame.render_widget(list, area);
}

fn render_sort(frame: &mut Frame, area: Rect, app: &App) {
    let mut bars = vec![];

    for i in 0..app.data.len() {
        let color = if app.current_screen == AppScreen::Menu {
            Color::DarkGray
        } else if app.n != 0 {
            if i < app.n {
                Color::Green
            } else {
                Color::White
            }
        } else{
            if app.swapped.contains(&app.data[i]) {
                Color::Red
            } else {
                Color::White
            }
        };

        bars.push(
            Bar::default()
                .value(app.data[i].try_into().unwrap())
                .style(color)
                .value_style(Style::new().bg(color).fg(color))
        );
    }

    let bar_chart = BarChart::default()
        .bar_gap(0)
        .data(BarGroup::default().bars(&bars));

    frame.render_widget(bar_chart, area);
}