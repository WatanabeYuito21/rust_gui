use iced::Element;
use iced::widget::{button, column, row, text};

#[derive(Default)]
struct Counter {
    value: u64,
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
}

fn view(counter: &Counter) -> Element<Message> {
    // button(text(counter)).on_press(Message::Increment).into()
    column![
        text("This is Text"),
        row![
            text(counter.value.to_string()),
            button("This is button").on_press(Message::Increment)
        ]
    ]
    .spacing(10)
    .into()
}

fn update(counter: &mut Counter, message: Message) {
    match message {
        Message::Increment => counter.value += 1,
    }
}

pub fn main() -> iced::Result {
    iced::run("This is title", update, view)
}
