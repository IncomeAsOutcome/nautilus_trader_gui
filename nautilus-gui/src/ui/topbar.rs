use iced::{
    widget::{button, container, row, text, Container, Row, Space, TextInput},
    Alignment, Element, Length,
};
use crate::app::Message;
use crate::models::AppState;

pub struct TopBar;

impl TopBar {
    pub fn new() -> Self {
        Self
    }

    pub fn view(&self, state: &AppState) -> Element<Message> {
        let symbol = state.selected_symbol.as_ref()
            .map(|s| s.as_str())
            .unwrap_or("Select Symbol");

        let account_info = format!("Balance: ${:.2}", state.account_balance);

        container(
            row![
                // Symbol selector
                button(text(symbol).size(14))
                    .padding([8, 16]),
                
                // Timeframe selector
                button(text(&state.selected_timeframe).size(14))
                    .padding([8, 16]),
                
                Space::with_width(Length::Fill),
                
                // Account info
                text(&account_info).size(14),
                
                Space::with_width(20),
                
                // Quick actions
                button(text("🔔").size(16))
                    .padding(8),
                button(text("👤").size(16))
                    .padding(8),
            ]
            .spacing(10)
            .align_y(Alignment::Center)
            .padding(10)
        )
        .width(Length::Fill)
        .height(Length::Fixed(60.0))
        .into()
    }
}