use iced::{
    widget::{button, column, container, row, text},
    Element, Length, Task, Theme,
};

pub struct SimpleApp {
    counter: i32,
}

#[derive(Debug, Clone)]
pub enum Message {
    Increment,
    Decrement,
}

impl SimpleApp {
    pub fn new() -> (Self, Task<Message>) {
        (Self { counter: 0 }, Task::none())
    }

    pub fn title(&self) -> String {
        String::from("NautilusTrader GUI - Demo")
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Increment => {
                self.counter += 1;
                Task::none()
            }
            Message::Decrement => {
                self.counter -= 1;
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        container(
            column![
                text("NautilusTrader GUI Demo").size(32),
                text(format!("Counter: {}", self.counter)).size(24),
                row![
                    button("Increment").on_press(Message::Increment),
                    button("Decrement").on_press(Message::Decrement),
                ]
                .spacing(10),
            ]
            .spacing(20)
            .padding(20)
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
    }

    pub fn theme(&self) -> Theme {
        Theme::Dark
    }
}