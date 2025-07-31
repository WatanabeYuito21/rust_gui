use iced::Element;
use iced::widget::{button, column, text};

#[derive(Debug, Clone)]
enum Message {
    Increment,
}

fn view(counter: &u64) -> Element<Message> {
    // button(text(counter)).on_press(Message::Increment).into()
    column![
        text(counter.to_string()),
        button("This is button").on_press(Message::Increment)
    ]
    .into()
}

fn update(counter: &mut u64, message: Message) {
    match message {
        Message::Increment => *counter += 1,
    }
}

pub fn main() -> iced::Result {
    iced::run("title", update, view)
}
