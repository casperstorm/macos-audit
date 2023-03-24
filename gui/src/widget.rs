#![allow(dead_code)]
use crate::theme::Theme;

pub type Renderer = iced::Renderer<Theme>;
pub type Element<'a, Message> = iced::Element<'a, Message, Renderer>;
pub type Column<'a, Message> = iced::widget::Column<'a, Message, Renderer>;
pub type Text<'a> = iced::widget::Text<'a, Renderer>;
