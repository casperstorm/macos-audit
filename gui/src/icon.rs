use iced::widget::text;
use iced::Font;

use crate::widget::Text;

const FONT: Font = Font::External {
    name: "Icons",
    bytes: include_bytes!("../fonts/icons.ttf"),
};

pub fn cloud_slash<'a>() -> Text<'a> {
    to_text('\u{f290}')
}

pub fn cloud_plus<'a>() -> Text<'a> {
    to_text('\u{f28e}')
}

pub fn cloud<'a>() -> Text<'a> {
    to_text('\u{f293}')
}

pub fn cloud_arrow_up<'a>() -> Text<'a> {
    to_text('\u{f285}')
}

pub fn cloud_checkmark<'a>() -> Text<'a> {
    to_text('\u{f287}')
}

fn to_text<'a>(unicode: char) -> Text<'a> {
    text(unicode.to_string()).font(FONT)
}
