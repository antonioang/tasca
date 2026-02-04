use crate::{db::expense_repo::ExpenseRepo, models::expense::Expense, tui::app_input::AppInput};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum AppMode {
    #[default]
    Normal,
    Editing,
    Creating,
    Detail,
}

pub struct AppState {
    pub idx: Option<usize>,
    pub expense_page: i64,
    pub expenses: Option<Vec<Expense>>,
    pub app_input: AppInput,
    pub app_mode: AppMode,
    pub status_message: Option<String>,
    pub detail_idx: Option<usize>,
}

impl AppState {
    pub fn new(er: &ExpenseRepo) -> AppState {
        //TODO: ok() discards error, is better to check if any
        // let first_10_expenses = er.get(1).ok();
        let first_10_expenses = match er.get(1) {
            Ok(v) => Some(v),
            Err(e) => {
                eprintln!("Errore: {}", e);
                None
            }
        };

        Self {
            idx: Some(0),
            expense_page: 1,
            app_input: AppInput::default(),
            expenses: first_10_expenses,
            app_mode: AppMode::Normal,
            status_message: None,
            detail_idx: None,
        }
    }
    pub fn start_editing(&mut self) {
        self.app_mode = AppMode::Editing
    }

    pub fn start_creating(&mut self) {
        self.app_input = AppInput::default();
        self.status_message = None;
        self.app_mode = AppMode::Creating
    }

    pub fn stop_editing(&mut self) {
        self.app_input = AppInput::default();
        self.app_mode = AppMode::Normal
    }

    pub fn start_detail(&mut self, idx: Option<usize>) {
        self.detail_idx = idx;
        self.app_mode = AppMode::Detail
    }

    pub fn stop_detail(&mut self) {
        self.detail_idx = None;
        self.app_mode = AppMode::Normal
    }
}
