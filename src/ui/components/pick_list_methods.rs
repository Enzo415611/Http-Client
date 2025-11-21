use iced::widget::{PickList, pick_list};

use crate::{App, Message, Methods};

impl App {
    pub fn pick_list_methods(&self) -> PickList<'_, Methods, [Methods; 6], Methods, Message> {
        let methods = [
            Methods::Get,
            Methods::Post,
            Methods::Put,
            Methods::Delete,
            Methods::Patch,
            Methods::Head,
        ];

        pick_list(
            methods,
            self.pick_list_method.clone(),
            Message::SelectedMethod,
        )
        .into()
    }
}
