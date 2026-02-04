use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
use time::{Date, macros::format_description};

use crate::app_state::{AppMode, AppState};

pub fn draw(frame: &mut Frame, app_state: &AppState) {
    let main_layout = Layout::vertical([Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(frame.area());
    let header_layout = Layout::vertical([Constraint::Length(3), Constraint::Min(0)])
        .split(main_layout[0]);

    match app_state.app_mode {
        AppMode::Creating | AppMode::Editing => {
            app_state.app_input.render(frame, header_layout[0], app_state.app_mode);
            if let Some(msg) = &app_state.status_message {
                let status = Paragraph::new(msg.as_str())
                    .block(Block::default().title("Stato").borders(Borders::ALL));
                frame.render_widget(status, header_layout[1]);
            } else {
                let help = Paragraph::new("Invio: salva  |  Esc: annulla")
                    .block(Block::default().title("Aiuto").borders(Borders::ALL));
                frame.render_widget(help, header_layout[1]);
            }
        }
        AppMode::Detail => {
            let detail = build_detail(app_state)
                .unwrap_or_else(|| "Nessuna spesa selezionata".to_string());
            let help = Paragraph::new(detail)
                .block(Block::default().title("Dettaglio spesa").borders(Borders::ALL));
            frame.render_widget(help, main_layout[0]);
        }
        AppMode::Normal => {
            let mut help_lines = vec![
                "a: aggiungi  |  Invio: dettaglio  |  j/k: muovi  |  h/l: pagina  |  q: esci"
                    .to_string(),
            ];
            if let Some(msg) = &app_state.status_message {
                help_lines.push(format!("Ultimo messaggio: {}", msg));
            }
            let help = Paragraph::new(help_lines.join("\n"))
                .block(Block::default().title("Comandi").borders(Borders::ALL));
            frame.render_widget(help, main_layout[0]);
        }
    }

    if let Some(expenses) = &app_state.expenses {
        let mut list_state = ListState::default();
        list_state.select(app_state.idx);

        let items: Vec<ListItem> = expenses
            .iter()
            .map(|e| {
                let date = format_date(e.date);
                let line = format!("{}  |  {:.2}  |  {}", e.description, e.amount, date);
                ListItem::new(line)
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().title("Expenses").borders(Borders::ALL))
            .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

        frame.render_stateful_widget(list, main_layout[1], &mut list_state);
    }
}

fn build_detail(app_state: &AppState) -> Option<String> {
    let idx = app_state.detail_idx?;
    let expenses = app_state.expenses.as_ref()?;
    let expense = expenses.get(idx)?;
    let date = format_date(expense.date);
    Some(format!(
        "ID: {}\nDescrizione: {}\nImporto: {:.2}\nData: {}",
        expense.id, expense.description, expense.amount, date
    ))
}

fn format_date(date: Option<Date>) -> String {
    match date {
        Some(d) => d
            .format(format_description!("[day]-[month]-[year]"))
            .unwrap_or_else(|_| "-".to_string()),
        None => "-".to_string(),
    }
}
