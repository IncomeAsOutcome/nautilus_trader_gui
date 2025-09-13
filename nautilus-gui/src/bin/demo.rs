use iced::{
    widget::{button, column, container, row, text, Column, Container, Space},
    Alignment, Element, Length, Task, Theme,
};
use rust_decimal::Decimal;
use std::str::FromStr;

fn main() -> iced::Result {
    iced::application("NautilusTrader GUI Demo", App::update, App::view)
        .theme(App::theme)
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
                // In a real app, this would fetch data from the backend
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let sidebar = self.sidebar();
        let main_content = match self.current_view {
            ViewType::Dashboard => self.dashboard_view(),
            ViewType::Trading => self.trading_view(),
            ViewType::Backtesting => self.backtesting_view(),
            ViewType::Settings => self.settings_view(),
        };

        container(
            row![sidebar, main_content]
                .spacing(0)
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn sidebar(&self) -> Container<Message> {
        let nav_button = |label: &str, view: ViewType| {
            let is_selected = self.current_view == view.clone();
            let btn = button(text(label).size(14))
                .width(Length::Fill)
                .on_press(Message::ViewChanged(view));
            
            if is_selected {
                btn.style(|_theme, _status| {
                    iced::widget::button::Style {
                        background: Some(iced::Background::Color(iced::Color::from_rgb(0.2, 0.3, 0.4))),
                        ..Default::default()
                    }
                })
            } else {
                btn
            }
        };

        container(
            column![
                text("NautilusTrader").size(20),
                Space::with_height(20),
                nav_button("📊 Dashboard", ViewType::Dashboard),
                nav_button("💹 Trading", ViewType::Trading),
                nav_button("📈 Backtesting", ViewType::Backtesting),
                nav_button("⚙️ Settings", ViewType::Settings),
            ]
            .spacing(5)
            .padding(10)
            .width(Length::Fixed(200.0))
        )
        .height(Length::Fill)
        .style(|_theme| {
            container::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgb(0.1, 0.1, 0.15))),
                ..Default::default()
            }
        })
    }

    fn dashboard_view(&self) -> Container<Message> {
        let total_pnl: Decimal = self.positions.iter()
            .map(|p| (p.current_price - p.entry_price) * p.quantity)
            .sum();

        container(
            column![
                text("Dashboard").size(28),
                Space::with_height(20),
                row![
                    self.stat_card("Balance", &format!("${}", self.balance)),
                    self.stat_card("Total P&L", &format!("${}", total_pnl)),
                    self.stat_card("Positions", &self.positions.len().to_string()),
                ]
                .spacing(20),
                Space::with_height(20),
                text("Recent Activity").size(20),
                container(
                    text("Trade history will appear here...")
                )
                .padding(20)
                .width(Length::Fill)
                .style(|_theme| {
                    container::Style {
                        background: Some(iced::Background::Color(iced::Color::from_rgb(0.15, 0.15, 0.2))),
                        border: iced::Border {
                            color: iced::Color::from_rgb(0.3, 0.3, 0.4),
                            width: 1.0,
                            radius: 5.0.into(),
                        },
                        ..Default::default()
                    }
                }),
            ]
            .spacing(10)
            .padding(20)
        )
        .width(Length::Fill)
        .height(Length::Fill)
    }

    fn trading_view(&self) -> Container<Message> {
        container(
            column![
                text("Trading").size(28),
                Space::with_height(20),
                row![
                    container(
                        column![
                            text(format!("Chart: {}", self.selected_symbol)).size(16),
                            container(
                                text("Chart will be rendered here with eGUI")
                            )
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .padding(20)
                            .style(|_theme| {
                                container::Style {
                                    background: Some(iced::Background::Color(iced::Color::from_rgb(0.1, 0.1, 0.15))),
                                    border: iced::Border {
                                        color: iced::Color::from_rgb(0.3, 0.3, 0.4),
                                        width: 1.0,
                                        radius: 5.0.into(),
                                    },
                                    ..Default::default()
                                }
                            }),
                        ]
                    )
                    .width(Length::FillPortion(3))
                    .height(Length::Fill),
                    
                    container(
                        column![
                            text("Order Panel").size(16),
                            Space::with_height(10),
                            button("Buy").width(Length::Fill),
                            button("Sell").width(Length::Fill),
                        ]
                        .spacing(10)
                    )
                    .width(Length::FillPortion(1))
                    .padding(10),
                ]
                .spacing(20)
            ]
            .padding(20)
        )
        .width(Length::Fill)
        .height(Length::Fill)
    }

    fn backtesting_view(&self) -> Container<Message> {
        container(
            column![
                text("Backtesting").size(28),
                Space::with_height(20),
                text("Strategy backtesting interface will be here"),
                Space::with_height(20),
                button("Run Backtest"),
            ]
            .padding(20)
        )
        .width(Length::Fill)
        .height(Length::Fill)
    }

    fn settings_view(&self) -> Container<Message> {
        container(
            column![
                text("Settings").size(28),
                Space::with_height(20),
                text("Application settings and configuration"),
            ]
            .padding(20)
        )
        .width(Length::Fill)
        .height(Length::Fill)
    }

    fn stat_card(&self, title: &str, value: &str) -> Container<Message> {
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
        .style(|_theme| {
            container::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgb(0.15, 0.15, 0.2))),
                border: iced::Border {
                    color: iced::Color::from_rgb(0.3, 0.3, 0.4),
                    width: 1.0,
                    radius: 5.0.into(),
                },
                ..Default::default()
            }
        })
    }
}