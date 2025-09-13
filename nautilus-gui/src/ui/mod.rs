use iced::{
    widget::{button, column, container, row, text, Column, Container, Row, Space},
    Alignment, Element, Length,
};
use crate::app::Message;
use crate::models::AppState;

mod sidebar;
mod topbar;
mod main_view;

pub use sidebar::Sidebar;
pub use topbar::TopBar;
pub use main_view::MainView;

pub fn styled_button<'a>(label: &'a str) -> iced::widget::Button<'a, Message> {
    button(text(label).size(14))
        .padding([8, 16])
}

pub fn card<'a>(content: impl Into<Element<'a, Message>>) -> Container<'a, Message> {
    container(content)
        .padding(20)
        .width(Length::Fill)
}