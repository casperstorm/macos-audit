use std::path::PathBuf;

use iced::widget::{column, container, text};
use iced::{Command, Length};

use crate::icon;
use crate::widget::Element;

#[derive(Default, Debug, Clone)]
pub enum Mode {
    #[default]
    Idle,
    Hovered(PathBuf),
    Dropped(PathBuf),
}

pub struct Drop {
    mode: Mode,
}

#[derive(Debug, Clone)]
pub enum Message {
    EventOccurred(iced::Event),
}

pub enum Event {
    Dropped(PathBuf),
}

impl Drop {
    pub fn new() -> Self {
        Self {
            mode: Mode::default(),
        }
    }

    pub fn update(&mut self, message: Message) -> Option<(Event, Command<Message>)> {
        match message {
            Message::EventOccurred(event) => {
                let iced::Event::Window(window) = event else {
                    return None;
                };

                match window {
                    iced::window::Event::FileHovered(path) => {
                        self.mode = Mode::Hovered(path);
                        None
                    }
                    iced::window::Event::FileDropped(path) => {
                        self.mode = Mode::Dropped(path.clone());
                        Some((Event::Dropped(path), Command::none()))
                    }
                    iced::window::Event::FilesHoveredLeft => {
                        self.mode = Mode::Idle;
                        None
                    }
                    _ => None,
                }
            }
        }
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        iced::subscription::events().map(Message::EventOccurred)
    }

    pub fn view(&self) -> Element<Message> {
        let (icon, message) = match &self.mode {
            Mode::Idle => (icon::cloud(), "Drop macOS application here"),
            Mode::Hovered(path) => {
                if data::verify::is_mac_application(path) {
                    (icon::cloud_arrow_up(), "Release to audit application")
                } else {
                    (icon::cloud_slash(), "This file is not supported")
                }
            }
            Mode::Dropped(_) => (icon::cloud_checkmark(), "Thanks!"),
        };

        container(column![icon.size(64), text(message)].align_items(iced::Alignment::Center))
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
