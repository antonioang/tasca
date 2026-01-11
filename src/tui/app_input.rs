#[allow(unused_imports)]
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Paragraph},
};
use tui_input::Input;

use crate::app_state::AppMode;

#[derive(Debug, Default)]
pub struct AppInput {
    pub input: Input,
}

impl AppInput {
    pub fn push_message<F>(&mut self, mut cb: F)
    where
        F: FnMut(String),
    {
        cb(self.input.value_and_reset());
    }

    pub fn render(&self, frame: &mut Frame, rect: Rect, input_mode: AppMode) {
        self.render_input(frame, rect, input_mode);
    }
    
    fn render_input(&self, frame: &mut Frame, area: Rect, input_mode: AppMode) {
        let width = area.width.max(3) - 3;
        let scroll = self.input.visual_scroll(width as usize);
        let style = match input_mode {
            AppMode::Normal => Style::default(),
            AppMode::Editing => Color::Yellow.into(),
            AppMode::Creating => Color::Yellow.into(),
        };
        let input = Paragraph::new(self.input.value())
            .style(style)
            .scroll((0, scroll as u16))
            .block(Block::bordered().title("Host"));
        frame.render_widget(input, area);

        if input_mode == AppMode::Editing {
            let x = self.input.visual_cursor().max(scroll) - scroll + 1;
            frame.set_cursor_position((area.x + x as u16, area.y + 1))
        }
    }
}
