#![allow(dead_code)]
mod app_state;
mod db;
mod models;
mod tui;

use std::time::Duration;

use app_state::AppState;
use color_eyre::{Result, eyre::Context};
use ratatui::{
    DefaultTerminal,
    crossterm::event,
};

use crate::{app_state::actions::AppAction, db::expense_repo, tui::{handler, view}};

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

    terminal.draw(|f| view::draw(f, &state))?;

    loop {
        terminal.draw(|f| view::draw(f, &state))?;
        if event::poll(Duration::from_millis(250)).context("event poll failed")? {
            let event = event::read().context("event read failed")?;
            match handler::handle_event(&mut state, &expense_repo, event)? {
                AppAction::Quit => break,
                AppAction::None => {}
            }
        }
    }
    Ok(())
}
