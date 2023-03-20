use iced::widget::{container, text};
use iced::{Command, Element};

pub struct Dashboard;

#[derive(Debug, Clone)]
pub enum Message {}

pub enum Event {}

impl Dashboard {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&mut self, message: Message) -> Option<(Event, Command<Message>)> {
        match message {}
    }

    pub fn view(&self) -> Element<Message> {
        container(text("tba")).into()
    }
}
