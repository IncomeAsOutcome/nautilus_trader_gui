use iced::{
    widget::{button, column, container, text},
    Element, Task, Theme,
};

fn main() -> iced::Result {
    iced::application("NautilusTrader GUI", update, view)
        .theme(|_| Theme::Dark)
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    ButtonPressed,
}

fn update(state: &mut u32, message: Message) -> Task<Message> {
    match message {
        Message::ButtonPressed => {
            *state += 1;
            Task::none()
        }
    }
}

fn view(state: &u32) -> Element<Message> {
    container(
        column![
            text(format!("NautilusTrader GUI - Count: {}", state)).size(24),
            button("Click me!").on_press(Message::ButtonPressed),
        ]
        .spacing(20)
    )
    .padding(20)
    .into()
}