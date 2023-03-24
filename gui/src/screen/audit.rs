use data::application;
use iced::widget::{column, container, scrollable, text};
use iced::{Command, Length};

use crate::widget::Element;

pub struct Audit {
    application: application::Application,
}

#[derive(Debug, Clone)]
pub enum Message {}

pub enum Event {}

impl Audit {
    pub fn new(application: &application::Application) -> Self {
        Self {
            application: application.clone(),
        }
    }

    pub fn update(&mut self, _message: Message) -> Option<(Event, Command<Message>)> {
        None
    }

    pub fn view(&self) -> Element<Message> {
        let column = column(
            self.application
                .entitlements
                .clone()
                .into_iter()
                .map(|(ent, _)| text(ent).into())
                .collect(),
        );

        container(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
