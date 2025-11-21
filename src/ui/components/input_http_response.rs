use iced::{Element, widget::text_editor};

use crate::{App, Message};

impl App {
    pub fn input_http_response(&self) -> Element<'_, Message> {
        text_editor(&self.http_response_editor)
            .placeholder("Http Response")
            .height(500.0)
            .on_action(Message::HttpResponseAction)
            .into()
    }
}
