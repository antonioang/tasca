use std::str::FromStr;

use color_eyre::Result;
use ratatui::crossterm::event::{Event, KeyCode};
use time::{Date, macros::format_description};
use tui_input::backend::crossterm::EventHandler;

use crate::{
    app_state::{AppMode, AppState},
    db::expense_repo::ExpenseRepo,
};

pub enum AppAction {
    None,
    Quit,
}

pub fn handle_event(
    state: &mut AppState,
    expense_repo: &ExpenseRepo,
    event: Event,
) -> Result<AppAction> {
    match state.app_mode {
        AppMode::Normal => handle_normal(state, expense_repo, event),
        AppMode::Editing => handle_editing(state, event),
        AppMode::Creating => handle_creating(state, expense_repo, event),
        AppMode::Detail => handle_detail(state, event),
    }
}

fn handle_normal(
    state: &mut AppState,
    expense_repo: &ExpenseRepo,
    event: Event,
) -> Result<AppAction> {
    if let Event::Key(key) = event {
        match key.code {
            KeyCode::Char('q') => return Ok(AppAction::Quit),
            KeyCode::Char('a') => state.start_creating(),
            KeyCode::Enter => state.start_detail(state.idx),
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
            }
            KeyCode::Char('k') => {
                if let Some(idx) = state.idx {
                    state.idx = Some(idx - 1)
                } else {
                    state.idx = Some(1)
                }
            }
            KeyCode::Char('l') => {
                state.expense_page = state.expense_page + 1;
                state.idx = None;
                refresh_expenses(state, expense_repo)?;
            }
            KeyCode::Char('h') => {
                state.expense_page = state.expense_page - 1;
                state.idx = None;
                refresh_expenses(state, expense_repo)?;
            }
            _ => {}
        }
    }
    Ok(AppAction::None)
}

fn handle_editing(state: &mut AppState, event: Event) -> Result<AppAction> {
    if let Event::Key(_key) = event {
        // todo!()
        let _ = state;
    }
    Ok(AppAction::None)
}

fn handle_creating(
    state: &mut AppState,
    expense_repo: &ExpenseRepo,
    event: Event,
) -> Result<AppAction> {
    match event {
        Event::Key(key) => match key.code {
            KeyCode::Esc => state.stop_editing(),
            KeyCode::Enter => {
                let mut status = None;
                let mut saved = false;
                state.app_input.push_message(|msg| {
                    let parts: Vec<&str> = msg.split('|').map(|s| s.trim()).collect();
                    if parts.len() != 3 {
                        status = Some("Formato: descrizione | importo | YYYY-MM-DD".to_string());
                        return;
                    }
                    let description = parts[0];
                    let amount_raw = parts[1];
                    let date_raw = parts[2];

                    let amount_clean = amount_raw.replace(',', ".");
                    if f64::from_str(&amount_clean).is_err() {
                        status = Some("Importo non valido (usa 10.50)".to_string());
                        return;
                    }
                    if Date::parse(date_raw, format_description!("[day]-[month]-[year]")).is_err() {
                        status = Some("Data non valida (DD-MM-YYYY)".to_string());
                        return;
                    }
                    if let Err(e) = expense_repo.insert(description, &amount_clean, date_raw) {
                        status = Some(format!("Errore inserimento: {}", e));
                        return;
                    }
                    status = Some("Spesa aggiunta".to_string());
                    saved = true;
                });
                state.status_message = status;
                if saved {
                    state.stop_editing();
                    refresh_expenses(state, expense_repo)?;
                }
            }
            _ => {
                state.app_input.input.handle_event(&event);
            }
        },
        _ => {}
    }
    Ok(AppAction::None)
}

fn handle_detail(state: &mut AppState, event: Event) -> Result<AppAction> {
    if let Event::Key(key) = event {
        match key.code {
            KeyCode::Esc | KeyCode::Enter => state.stop_detail(),
            _ => {}
        }
    }
    Ok(AppAction::None)
}

fn refresh_expenses(state: &mut AppState, expense_repo: &ExpenseRepo) -> Result<()> {
    state.expenses = match expense_repo.get(state.expense_page) {
        Ok(v) => Some(v),
        Err(e) => {
            eprintln!("Errore: {}", e);
            None
        }
    };
    Ok(())
}
