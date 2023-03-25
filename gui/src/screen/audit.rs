use data::application;
use iced::widget::{button, column, container, horizontal_space, row, scrollable, text};
use iced::{Command, Length};

use crate::font::BOLD;
use crate::widget::Element;
use crate::{icon, theme};

pub struct Audit {
    application: application::Application,
}

#[derive(Debug, Clone)]
pub enum Message {
    BackPressed,
}

pub enum Event {
    GoBack,
}

impl Audit {
    pub fn new(application: &application::Application) -> Self {
        Self {
            application: application.clone(),
        }
    }

    pub fn update(&mut self, message: Message) -> Option<(Event, Command<Message>)> {
        match message {
            Message::BackPressed => Some((Event::GoBack, Command::none())),
        }
    }

    pub fn view(&self) -> Element<Message> {
        let header: Element<Message> = {
            row![
                button(icon::arrow_left())
                    .style(theme::Button::Default)
                    .on_press(Message::BackPressed),
                horizontal_space(Length::Fill),
                text(&self.application).font(BOLD).size(16),
                horizontal_space(Length::Fill),
            ]
            .align_items(iced::Alignment::Center)
            .into()
        };

        let entitlements = column![
            text("Entitlements").font(BOLD),
            column(
                self.application
                    .entitlements
                    .clone()
                    .into_iter()
                    .map(|(ent, _)| text(ent).into())
                    .collect(),
            )
        ]
        .spacing(4)
        .width(Length::Fill);

        container(
            column![
                header,
                scrollable(entitlements)
                    .vertical_scroll(scrollable::Properties::default().scroller_width(1).width(1))
            ]
            .spacing(4),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
