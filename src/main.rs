use iced::Element;
use iced::widget::{button, column, row, text, text_input};

#[derive(Default)]
struct Counter {
    value: u64,
    content: String,
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
    ContentChanged(String),
}

fn view(counter: &Counter) -> Element<Message> {
    column![
        text_input("Type something hire...", &counter.content).on_input(Message::ContentChanged),
        text(format!("This is Text: {}", counter.content)),
        row![
            button("This is button").on_press(Message::Increment),
            text(format!("This is counter: {}", counter.value)),
        ]
        .spacing(10),
    ]
    .spacing(10)
    .into()
}

fn update(counter: &mut Counter, message: Message) {
    match message {
        Message::Increment => counter.value += 1,
        Message::ContentChanged(content) => {
            counter.content = content;
        }
    }
}

pub fn main() -> iced::Result {
    iced::run("This is title", update, view)
}
