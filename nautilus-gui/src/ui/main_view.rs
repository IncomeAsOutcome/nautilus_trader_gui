use iced::{
    widget::{column, container, row, text, Column, Container, Row, Space},
    Alignment, Element, Length,
};
use crate::app::Message;
use crate::models::AppState;
use rust_decimal::Decimal;

pub struct MainView;

impl MainView {
    pub fn new() -> Self {
        Self
    }

    pub fn dashboard_view(&self, state: &AppState) -> Element<Message> {
        let total_positions = state.positions.len();
        let open_orders = state.orders.iter()
            .filter(|o| o.status == crate::models::OrderStatus::Submitted)
            .count();
        
        let total_pnl: Decimal = state.positions.iter()
            .map(|p| p.unrealized_pnl)
            .sum();

        column![
            text("Dashboard").size(28),
            Space::with_height(20),
            
            // Stats cards row
            row![
                self.stat_card("Account Balance", &format!("${:.2}", state.account_balance)),
                self.stat_card("Total P&L", &format!("${:.2}", total_pnl)),
                self.stat_card("Open Positions", &total_positions.to_string()),
                self.stat_card("Open Orders", &open_orders.to_string()),
            ]
            .spacing(20),
            
            Space::with_height(30),
            
            // Recent activity
            text("Recent Activity").size(20),
            Space::with_height(10),
            container(
                column![
                    text("Recent trades and activities will appear here"),
                ]
                .spacing(10)
            )
            .padding(20)
            .width(Length::Fill)
            .height(Length::Fixed(200.0)),
            
            Space::with_height(30),
            
            // Market overview
            text("Market Overview").size(20),
            Space::with_height(10),
            container(
                column![
                    text("Market trends and watchlist will appear here"),
                ]
                .spacing(10)
            )
            .padding(20)
            .width(Length::Fill)
            .height(Length::Fixed(200.0)),
        ]
        .spacing(10)
        .into()
    }

    fn stat_card<'a>(&self, title: &str, value: &str) -> Container<'a, Message> {
        container(
            column![
                text(title).size(14),
                Space::with_height(10),
                text(value).size(24),
            ]
            .align_x(Alignment::Center)
        )
        .padding(20)
        .width(Length::FillPortion(1))
        .height(Length::Fixed(100.0))
    }
}