use crate::{App, Message};

use iced::{Element, widget::text_editor};

impl App {
    pub fn input_json_body(&self) -> Element<'_, Message> {
        text_editor(&self.body_input_content)
            .placeholder("Json Body")
            .height(150.0)
            .size(15.0)
            .on_action(Message::BodyInput)
            .into()
    }
}
