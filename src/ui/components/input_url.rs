use iced::{Element, widget::text_input};

use crate::{App, Message};

impl App {
    pub fn input_url(&self) -> Element<'_, Message> {
        text_input("URL", &self.url)
            .on_input(Message::UrlInput)
            .into()
    }
}
