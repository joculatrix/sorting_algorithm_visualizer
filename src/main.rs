use std::{error::Error, io, time::Duration};

use app::{App, AppScreen};
use ratatui::{
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
    },
    prelude::{Backend, CrosstermBackend},
    Terminal
};
use sort::{shuffle, SortResult};

mod app;
mod sort;
mod ui;

pub const ARRAY_LEN: usize = 100;

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    run_app(&mut terminal, &mut app)?;

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<(), Box<dyn Error>> {
    loop {
        terminal.draw(|f| ui::ui(f, app))?;

        if app.current_screen == AppScreen::Sort {
            if let Some(ref mut sort) = app.sort {
                match sort.step(&mut app.data) {
                    SortResult::Done => {
                        app.sort.take();
                        app.swapped.clear();
                    }
                    SortResult::Swap(vec) => app.swapped = vec,
                    SortResult::Ok => app.swapped.clear(),
                }  
            };
        }
        
        if event::poll(Duration::from_millis(8))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Release { continue; }
                match app.current_screen {
                    AppScreen::Menu => match key.code {
                        KeyCode::Enter => {
                            app.sort = Some((app.algorithms[app.selected].new)());
                            shuffle(&mut app.data);
                            app.current_screen = AppScreen::Sort;
                        }
                        KeyCode::Esc => {
                            return Ok(());
                        }
                        KeyCode::Up => {
                            if app.selected == 0 {
                                app.selected = app.algorithms.len() - 1;
                            } else {
                                app.selected -= 1;
                            }
                        }
                        KeyCode::Down => {
                            if app.selected == app.algorithms.len() - 1 {
                                app.selected = 0;
                            } else {
                                app.selected += 1;
                            }
                        }
                        _ => ()
                    }
                    AppScreen::Sort => match key.code {
                        KeyCode::Esc => {
                            app.current_screen = AppScreen::Menu;
                        }
                        _ => ()
                    }
                }
            }
        }
    }
}
