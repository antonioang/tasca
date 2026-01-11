#![allow(dead_code)]
mod app_state;
mod db;
mod models;
mod tui;

use std::time::Duration;

use app_state::AppState;
use color_eyre::{Result, eyre::Context};
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    DefaultTerminal, Frame, layout::{Constraint, Layout}, style::{Color, Style}, widgets::{Block, Borders, List, ListItem, ListState}
};

use crate::{app_state::AppMode, db::expense_repo};

fn main() -> Result<()> {
    color_eyre::install()?;

    let terminal = ratatui::init();
    let app_result = run(terminal).context("app loop failed");

    ratatui::restore();
    app_result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let pool = db::init_db_connection("tasca.db")?;
    let expense_repo = expense_repo::ExpenseRepo::new(pool);
    let mut state = AppState::new(&expense_repo);

    // expense_repo.insert("Just a test description", "221", "22-12-2025")?;

    terminal.draw(|f| draw(f, &state))?;

    loop {
        terminal.draw(|f| draw(f, &state))?;
        if event::poll(Duration::from_millis(250)).context("event poll failed")? {
            let event = event::read().context("event read failed")?;
            if let Event::Key(key) = event {
                match state.app_mode {
                    AppMode::Normal => match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('d') => {
                            let id = "1";
                            expense_repo.delete(&id)?
                        }
                        KeyCode::Char('j') => {
                            if let Some(idx) = state.idx {
                                state.idx = Some(idx + 1)
                            } else {
                                state.idx = Some(1)
                            }
                        },
                        KeyCode::Char('k') => {
                            if let Some(idx) = state.idx {
                                state.idx = Some(idx - 1)
                            } else {
                                state.idx = Some(1)
                            }
                        },
                        KeyCode::Char('l') => {
                            //TODO: enhance get in order to not repeat check on result everywhere
                            state.expense_page = state.expense_page + 1;
                            state.idx = None;
                            state.expenses = match expense_repo.get(state.expense_page) {
                                Ok(v) => Some(v),
                                Err(e) => {
                                    eprintln!("Errore: {}", e);
                                    None
                                }
                            };
                        }
                        KeyCode::Char('h') => {
                            state.expense_page = state.expense_page - 1;
                            state.idx = None;
                            state.expenses = match expense_repo.get(state.expense_page) {
                                Ok(v) => Some(v),
                                Err(e) => {
                                    eprintln!("Errore: {}", e);
                                    None
                                }
                            };
                        }
                        _ => {
                            // todo!()
                        }
                    },
                    AppMode::Editing => match key.code {
                        _ => {
                            // todo!()
                        }
                    },
                    AppMode::Creating => match key.code {
                        _ => {
                            // todo!()
                        }
                    },
                }
            }
        }
    }
    Ok(())
}

fn draw(frame: &mut Frame, app_state: &AppState) {
    let main_layout = Layout::vertical([Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(frame.area());
    // println!("Found expense {:?}", &app_state.expenses);

    if let Some(expenses) = &app_state.expenses {
        let mut list_state = ListState::default();
        list_state.select(app_state.idx);

        let items: Vec<ListItem> = expenses
            .iter()
            .map(|e| ListItem::new(e.description.as_str()))
            .collect();

        let list = List::new(items)
            .block(Block::default().title("Expenses").borders(Borders::ALL))
            .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

        frame.render_stateful_widget(list, main_layout[1], &mut list_state);
    }
}
