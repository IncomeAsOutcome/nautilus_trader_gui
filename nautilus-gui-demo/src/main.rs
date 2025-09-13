use iced::{
    widget::{button, column, container, row, text, Column, Container, Space},
    Alignment, Element, Length, Task, Theme,
};
use rust_decimal::Decimal;
use std::str::FromStr;

fn main() -> iced::Result {
    println!("🚀 Starting NautilusTrader GUI Demo...");
    println!("📊 Modern GUI for algorithmic trading");
    println!("🎨 Built with Iced and eGUI\n");
    
    iced::application("NautilusTrader GUI", App::update, App::view)
        .theme(App::theme)
        .window_size((1600.0, 900.0))
        .run_with(App::new)
}

struct App {
    current_view: ViewType,
    selected_symbol: String,
    balance: Decimal,
    positions: Vec<Position>,
}

#[derive(Debug, Clone, PartialEq)]
enum ViewType {
    Dashboard,
    Trading,
    Backtesting,
    Settings,
}

#[derive(Debug, Clone)]
struct Position {
    symbol: String,
    quantity: Decimal,
    entry_price: Decimal,
    current_price: Decimal,
}

#[derive(Debug, Clone)]
enum Message {
    ViewChanged(ViewType),
    RefreshData,
}

impl App {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                current_view: ViewType::Dashboard,
                selected_symbol: "BTC/USD".to_string(),
                balance: Decimal::from_str("100000").unwrap(),
                positions: vec![
                    Position {
                        symbol: "BTC/USD".to_string(),
                        quantity: Decimal::from_str("0.5").unwrap(),
                        entry_price: Decimal::from_str("45000").unwrap(),
                        current_price: Decimal::from_str("47000").unwrap(),
                    },
                ],
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ViewChanged(view) => {
                self.current_view = view;
                Task::none()
            }
            Message::RefreshData => {
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        container(
            text("NautilusTrader GUI Demo").size(32)
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}