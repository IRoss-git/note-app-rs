use iced::widget::{button, column, text, Column, Container, Button, Text};

use iced::{ Settings, Result, Sandbox};

fn main() -> iced::Result {
    println!("Hello, world!");
    Counter::run(Settings::default())
}   

struct Counter {
    value: i32
}
#[derive(Debug, Clone, Copy)]
pub enum CMessage {
    IncrementPressed,
    DecrementPressed,
}
impl Sandbox for Counter {

    type Message = CMessage;
    fn new() -> Self {
        Counter { value: 0 }
    }

    fn title(&self) -> String {
        String::from("Counter app")
    }

    fn update(&mut self, message: CMessage) {
        match message {
            CMessage::IncrementPressed => {
                self.value += 1;
            }
            CMessage::DecrementPressed => {
                self.value -= 1;
            }
        }
    }

     fn view(&self) -> iced::Element<Self::Message> {
        let label = Text::new(format!("Count: {}", self.value));
        let incr = Button::new("Increment").on_press(CMessage::IncrementPressed);
        let decr = Button::new("Decrement").on_press(CMessage::DecrementPressed);
        let col = Column::new().push(incr).push(label).push(decr);
        Container::new(col).center_x().center_y().width(iced::Length::Fill).height(iced::Length::Fill).into()
    }
}

