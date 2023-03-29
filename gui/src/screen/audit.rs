use data::application;
use iced::widget::{button, column, container, horizontal_space, row, scrollable, text};
use iced::{Command, Length};

use crate::font::BOLD;
use crate::widget::Element;
use crate::{icon, theme};

pub struct Audit {
    application: application::Application,
    selected: Option<usize>,
}

#[derive(Debug, Clone)]
pub enum Message {
    EntitlementPressed(usize),
    BackPressed,
}

pub enum Event {
    GoBack,
}

impl Audit {
    pub fn new(application: &application::Application) -> Self {
        Self {
            application: application.clone(),
            selected: None,
        }
    }

    pub fn update(&mut self, message: Message) -> Option<(Event, Command<Message>)> {
        match message {
            Message::BackPressed => Some((Event::GoBack, Command::none())),
            Message::EntitlementPressed(idx) => {
                if Some(idx) == self.selected {
                    self.selected = None;
                } else {
                    self.selected = Some(idx);
                }

                None
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let controls = row![
            horizontal_space(Length::Fill),
            text(&self.application).font(BOLD).size(16),
            horizontal_space(Length::Fill),
            button(icon::arrow_left()).on_press(Message::BackPressed),
        ]
        .padding([2, 4])
        .align_items(iced::Alignment::Center);

        let entitlements = column![
            container(text("Entitlements").font(BOLD)).padding([4, 6]),
            column(
                self.application
                    .entitlements
                    .clone()
                    .into_iter()
                    .enumerate()
                    .map(|(idx, (ent, value))| {
                        let mut content = column![row![
                            text(ent),
                            horizontal_space(Length::Fill),
                            button(if Some(idx) == self.selected {
                                icon::arrow_up_short()
                            } else {
                                icon::arrow_down_short()
                            })
                            .on_press(Message::EntitlementPressed(idx)),
                        ]
                        .align_items(iced::Alignment::Center)];

                        if self.selected == Some(idx) {
                            content = content.push(text(value));
                        }

                        container(content)
                            .style(if idx % 2 == 0 {
                                theme::Container::Row(theme::Row::Even)
                            } else {
                                theme::Container::Row(theme::Row::Odd)
                            })
                            .padding([2, 4])
                            .width(Length::Fill)
                            .into()
                    })
                    .collect()
            )
            .spacing(0)
        ]
        .spacing(4)
        .width(Length::Fill);

        container(
            column![
                controls,
                scrollable(entitlements)
                    .vertical_scroll(scrollable::Properties::default().scroller_width(1).width(1))
            ]
            .spacing(4),
        )
        .padding([2, 0])
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
