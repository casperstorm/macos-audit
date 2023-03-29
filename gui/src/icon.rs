use iced::widget::text;

use crate::font::ICON;
use crate::widget::Text;

pub fn arrow_left<'a>() -> Text<'a> {
    to_text('\u{f12f}')
}

pub fn arrow_down_short<'a>() -> Text<'a> {
    to_text('\u{f124}')
}

pub fn arrow_up_short<'a>() -> Text<'a> {
    to_text('\u{f145}')
}

pub fn file_plus<'a>() -> Text<'a> {
    to_text('\u{f346}')
}

pub fn file_arrow_up<'a>() -> Text<'a> {
    to_text('\u{f321}')
}

pub fn file_cross<'a>() -> Text<'a> {
    to_text('\u{f358}')
}

fn to_text<'a>(unicode: char) -> Text<'a> {
    text(unicode.to_string()).font(ICON)
}
