//! GUI components for rpgmap-gui

use iced::{button, Element, Row};

/// Internal GUI <-> State messages
#[derive(Debug, Clone)]
pub enum Message {
}

#[derive(Default)]
pub struct Controls {
    generate_button: button::State,
}

impl Controls {
    pub fn view<'a>(&'a mut self) -> Element<'a, Message> {
        Row::new()
            .padding(10)
            .spacing(20)
            .into()
    }

}
