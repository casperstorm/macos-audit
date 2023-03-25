use iced::Font;

pub const ICON: Font = Font::External {
    name: "Icons",
    bytes: include_bytes!("../fonts/icons.ttf"),
};

pub const BOLD: Font = Font::External {
    name: "Iosevka Term Bold",
    bytes: include_bytes!("../fonts/iosevka-term-bold.ttf"),
};
