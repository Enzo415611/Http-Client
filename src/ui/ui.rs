use iced::{
    Element, Length,
    widget::{Column, button, column, row},
};

use crate::ui::style::style_container;
use crate::{App, Message};

impl App {
    pub fn view(&self) -> Element<'_, Message> {
        let col1: Column<Message> = column![
            row![
                self.input_url(),
                self.pick_list_methods(),
                button("Send").on_press(Message::SendButton)
            ],
            self.input_json_body(),
            self.input_http_response(),
        ]
        .spacing(20.0)
        .padding(10.0)
        .into();

        iced::widget::container(col1)
            .style(|_theme| style_container())
            .height(Length::Fill)
            .into()
    }
}
