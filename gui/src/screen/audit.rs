use data::entitlement;
use iced::widget::{container, text};
use iced::{Command, Length};

use crate::widget::Element;

pub struct Audit {}

#[derive(Debug, Clone)]
pub enum Message {}

pub enum Event {}

impl Audit {
    pub fn new(entitlements: &Result<entitlement::EntitlementList, entitlement::Error>) -> Self {
        println!("{:?}", entitlements);
        Self {}
    }

    pub fn update(&mut self, message: Message) -> Option<(Event, Command<Message>)> {
        None
    }

    pub fn view(&self) -> Element<Message> {
        container("hi")
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
