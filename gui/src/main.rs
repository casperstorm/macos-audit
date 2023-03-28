use iced::widget::container;
use iced::{executor, Application, Command, Length};
use widget::Element;

use self::screen::{audit, drop};

mod font;
mod icon;
mod screen;
mod theme;
mod widget;

fn main() -> iced::Result {
    if let Err(error) = Audit::run(settings()) {
        Err(error)
    } else {
        Ok(())
    }
}

fn settings() -> iced::Settings<()> {
    iced::Settings {
        default_font: Some(include_bytes!("../fonts/iosevka-term-regular.ttf")),
        default_text_size: 16.0,
        window: iced::window::Settings {
            size: (450, 450),
            platform_specific: iced::window::PlatformSpecific {
                title_hidden: true,
                titlebar_transparent: true,
                fullsize_content_view: true,
            },
            ..Default::default()
        },
        ..Default::default()
    }
}

struct Audit {
    screen: Screen,
}

enum Screen {
    Drop(screen::Drop),
    Audit(screen::Audit),
}

#[derive(Debug)]
enum Message {
    EventOccurred(iced::Event),
    Drop(drop::Message),
    Audit(audit::Message),
}

impl Application for Audit {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = theme::Theme;

    fn new(_flags: ()) -> (Audit, Command<Message>) {
        let screen = screen::Drop::new();

        (
            Audit {
                screen: Screen::Drop(screen),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Audit")
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        match &self.screen {
            Screen::Drop(dashboard) => dashboard.subscription().map(Message::Drop),
            _ => iced::subscription::events().map(Message::EventOccurred),
        }
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Drop(message) => {
                if let Screen::Drop(drop) = &mut self.screen {
                    if let Some((event, _command)) = drop.update(message) {
                        match event {
                            drop::Event::Dropped(application) => {
                                self.screen = Screen::Audit(screen::Audit::new(&application))
                            }
                        }
                    }
                }

                Command::none()
            }
            Message::Audit(message) => {
                if let Screen::Audit(audit) = &mut self.screen {
                    if let Some((event, _command)) = audit.update(message) {
                        match event {
                            audit::Event::GoBack => self.screen = Screen::Drop(screen::Drop::new()),
                        }
                    }
                }

                Command::none()
            }
            Message::EventOccurred(_) => Command::none(),
        }
    }

    fn view(&self) -> Element<Message> {
        let screen = match &self.screen {
            Screen::Drop(drop) => drop.view().map(Message::Drop),
            Screen::Audit(audit) => audit.view().map(Message::Audit),
        };

        container(screen)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(theme::Container::Default)
            .into()
    }
}
