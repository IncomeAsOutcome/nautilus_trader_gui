use iced::{
    widget::{button, column, container, text, Column, Container, Space},
    Alignment, Element, Length,
};
use crate::app::{Message, ViewType};
use crate::models::AppState;

pub struct Sidebar;

impl Sidebar {
    pub fn new() -> Self {
        Self
    }

    pub fn view(&self, state: &AppState) -> Element<Message> {
        let nav_items = vec![
            ("📊", "Dashboard", ViewType::Dashboard),
            ("💹", "Trading", ViewType::Trading),
            ("📈", "Backtesting", ViewType::Backtesting),
            ("🧪", "Strategy Dev", ViewType::StrategyDevelopment),
            ("💼", "Portfolio", ViewType::Portfolio),
            ("⚙️", "Settings", ViewType::Settings),
        ];

        let mut sidebar_content = Column::new()
            .spacing(5)
            .padding(10)
            .width(Length::Fixed(200.0));

        // Logo/Title
        sidebar_content = sidebar_content.push(
            container(
                column![
                    text("NautilusTrader").size(18),
                    text("GUI").size(14),
                    Space::with_height(20)
                ]
                .align_x(Alignment::Center)
            )
            .width(Length::Fill)
        );

        // Navigation items
        for (icon, label, view_type) in nav_items {
            let is_selected = state.current_view == view_type;
            
            let btn = if is_selected {
                button(
                    container(
                        iced::widget::row![
                            text(icon).size(20),
                            text(label).size(14)
                        ]
                        .spacing(10)
                        .align_y(Alignment::Center)
                    )
                    .width(Length::Fill)
                    .padding([10, 15])
                )
                .width(Length::Fill)
                .on_press(Message::ViewChanged(view_type))
            } else {
                button(
                    container(
                        iced::widget::row![
                            text(icon).size(20),
                            text(label).size(14)
                        ]
                        .spacing(10)
                        .align_y(Alignment::Center)
                    )
                    .width(Length::Fill)
                    .padding([10, 15])
                )
                .width(Length::Fill)
                .on_press(Message::ViewChanged(view_type))
            };

            sidebar_content = sidebar_content.push(btn);
        }

        // Connection status
        sidebar_content = sidebar_content.push(Space::with_height(Length::Fill));
        
        let status_icon = if state.is_connected { "🟢" } else { "🔴" };
        let status_text = if state.is_connected { "Connected" } else { "Disconnected" };
        
        sidebar_content = sidebar_content.push(
            container(
                iced::widget::row![
                    text(status_icon).size(12),
                    text(status_text).size(12)
                ]
                .spacing(5)
                .align_items(Alignment::Center)
            )
            .padding(10)
        );

        container(sidebar_content)
            .height(Length::Fill)
            .into()
    }
}