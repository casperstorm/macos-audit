use iced::widget::container;
use iced::{executor, Application, Command, Length};
use widget::Element;

use self::screen::dashboard;

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
    Dashboard(screen::Dashboard),
}

#[derive(Debug)]
enum Message {
    Dashboard(dashboard::Message),
}

impl Application for Audit {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = theme::Theme;

    fn new(_flags: ()) -> (Audit, Command<Message>) {
        let screen = screen::Dashboard::new();

        (
            Audit {
                screen: Screen::Dashboard(screen),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Audit")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Dashboard(message) => match &mut self.screen {
                Screen::Dashboard(dashboard) => {
                    if let Some((_event, _command)) = dashboard.update(message) {
                        // Handle events and commands.
                    }
                }
            },
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let screen = match &self.screen {
            Screen::Dashboard(dashboard) => dashboard.view().map(Message::Dashboard),
        };

        container(screen)
            .padding([30, 6, 6, 6])
            .width(Length::Fill)
            .height(Length::Fill)
            .style(theme::Container::Default)
            .into()
    }
}
