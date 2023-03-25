use std::path::PathBuf;

use data::application;
use iced::widget::{column, container, text};
use iced::{Command, Length};

use crate::icon;
use crate::widget::Element;

#[derive(Default, Debug)]
pub enum Mode {
    #[default]
    Idle,
    Hovered(PathBuf),
    Error(application::Error),
}

pub struct Drop {
    mode: Mode,
}

#[derive(Debug, Clone)]
pub enum Message {
    EventOccurred(iced::Event),
}

pub enum Event {
    Dropped(application::Application),
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
                        match application::Application::try_from(path.as_path()) {
                            Ok(application) => Some((Event::Dropped(application), Command::none())),
                            Err(error) => {
                                self.mode = Mode::Error(error);
                                None
                            }
                        }
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
            Mode::Idle => (icon::file_plus(), "Drop macOS application here".to_string()),
            Mode::Hovered(_) => (
                icon::file_arrow_up(),
                "Release to audit application".to_string(),
            ),
            Mode::Error(error) => (icon::file_cross(), error.to_string()),
        };

        container(
            column![icon.size(58), text(message)]
                .align_items(iced::Alignment::Center)
                .spacing(6),
        )
        .center_x()
        .center_y()
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
