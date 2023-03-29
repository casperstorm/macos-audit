use iced::widget::{button, container, scrollable, text};
use iced::{application, color, Background, Color};

#[derive(Debug, Clone, Copy, Default)]
pub struct Theme;

impl application::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: color!(0x28, 0x28, 0x28),
            text_color: color!(0xeb, 0xdb, 0xb2),
        }
    }
}

impl text::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: Self::Style) -> text::Appearance {
        text::Appearance {
            color: color!(0xeb, 0xdb, 0xb2).into(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Row {
    Odd,
    Even,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Container {
    #[default]
    Default,
    Row(Row),
}

impl container::StyleSheet for Theme {
    type Style = Container;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        match style {
            Container::Default => container::Appearance::default(),
            Container::Row(row) => match row {
                Row::Odd => container::Appearance {
                    background: Some(Background::Color(color!(0x50, 0x50, 0x50, 0.9))),
                    ..Default::default()
                },
                Row::Even => container::Appearance {
                    background: Some(Background::Color(color!(0x38, 0x38, 0x38, 0.9))),
                    ..Default::default()
                },
            },
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Scrollable {
    #[default]
    Default,
}

impl scrollable::StyleSheet for Theme {
    type Style = Scrollable;

    fn active(&self, style: &Self::Style) -> scrollable::Scrollbar {
        match style {
            Scrollable::Default => scrollable::Scrollbar {
                background: None,
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
                scroller: scrollable::Scroller {
                    color: color!(0xeb, 0xdb, 0xb2),
                    border_radius: 8.0,
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                },
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> scrollable::Scrollbar {
        match style {
            Scrollable::Default => scrollable::Scrollbar {
                ..self.active(style)
            },
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Button {
    #[default]
    Default,
    Bare,
}
impl button::StyleSheet for Theme {
    type Style = Button;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        match style {
            Button::Default => button::Appearance {
                background: None,
                ..Default::default()
            },
            Button::Bare => button::Appearance {
                background: None,
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
                ..Default::default()
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        match style {
            Button::Default => button::Appearance {
                background: Some(Background::Color(color!(0xeb, 0xdb, 0xb2, 0.2))),
                border_radius: 4.0,
                ..Default::default()
            },
            Button::Bare => button::Appearance {
                background: None,
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
                ..Default::default()
            },
        }
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        match style {
            Button::Default => button::Appearance {
                ..Default::default()
            },
            Button::Bare => button::Appearance {
                background: None,
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
                ..Default::default()
            },
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        match style {
            Button::Default => button::Appearance {
                background: Some(Background::Color(color!(0xeb, 0xdb, 0xb2, 0.35))),
                border_radius: 4.0,
                ..Default::default()
            },
            Button::Bare => button::Appearance {
                background: None,
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
                ..Default::default()
            },
        }
    }
}
