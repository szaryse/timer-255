use iced::widget::{button, column, row, text};
use iced::{window, Alignment, Element, Length, Sandbox, Settings};

fn main() -> iced::Result {
    Timer255::run(Settings {
        window: window::Settings {
            size: (360, 360),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

struct Timer255 {
    session_time: i32,
    break_time: i32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementSession,
    DecrementSession,
    IncrementBreak,
    DecrementBreak,
}

impl Sandbox for Timer255 {
    type Message = Message;

    fn new() -> Self {
        Self {
            session_time: 25,
            break_time: 5,
        }
    }

    fn title(&self) -> String {
        String::from("Timer255 - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementSession => {
                self.session_time += 1;
            }
            Message::DecrementSession => {
                self.session_time -= 1;
            }
            Message::IncrementBreak => {
                self.break_time += 1;
            }
            Message::DecrementBreak => {
                self.break_time -= 1;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        // TODO: components
        let label = text("Break Length").size(24);

        row![
            column![
                label,
                row![
                    button("-").on_press(Message::DecrementBreak),
                    text(self.break_time).size(24),
                    button("+").on_press(Message::IncrementBreak),
                ]
            ]
            .width(Length::Fill)
            .align_items(Alignment::Start),
            column![
                text("Session Length").size(24),
                row![
                    button("-").on_press(Message::DecrementSession),
                    text(self.session_time).size(24),
                    button("+").on_press(Message::IncrementSession),
                ]
            ]
            .width(Length::Fill)
            .align_items(Alignment::End)
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .into()
    }
}
