#![windows_subsystem = "windows"]

use std::fmt::Display;

use iced::{
    Task, Theme,
    widget::text_editor::{self, Content},
};

use crate::http::run_http_method;

mod http;
mod ui;

fn main() -> iced::Result {
    iced::application("Http Client", App::update, App::view)
        .theme(|_a| Theme::Dark)
        .antialiasing(true)
        .run()
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Methods {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
}

impl Display for Methods {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Methods::Get => "Get",
            Methods::Post => "Post",
            Methods::Put => "Put",
            Methods::Delete => "Delete",
            Methods::Patch => "Patch",
            Methods::Head => "Head",
        })
    }
}

#[derive(Debug, Clone)]
enum Message {
    SendButton,
    SelectedMethod(Methods),
    HttpMethod(String),
    UrlInput(String),
    BodyInput(text_editor::Action),
    HttpResponseAction(text_editor::Action),
}

#[derive()]
struct App {
    url: String,
    selected_method: Methods,
    http_response: String,
    pick_list_method: Option<Methods>,
    http_response_editor: text_editor::Content,
    body_input_content: text_editor::Content,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    fn new() -> Self {
        Self {
            url: "".to_string(),
            selected_method: Methods::Get,
            http_response: "".to_string(),
            pick_list_method: Some(Methods::Get),
            http_response_editor: Content::new(),
            body_input_content: Content::new(),
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SendButton => {
                let url: String = self.url.clone();
                self.http_response_editor
                    .perform(text_editor::Action::SelectAll);
                self.http_response_editor
                    .perform(text_editor::Action::Edit(text_editor::Edit::Delete));

                Task::perform(
                    run_http_method(
                        url,
                        self.selected_method.clone(),
                        self.body_input_content.text(),
                    ),
                    Message::HttpMethod,
                )
            }
            Message::SelectedMethod(method) => {
                self.pick_list_method = Some(method.clone());
                self.selected_method = method;
                Task::none()
            }
            Message::HttpMethod(text) => {
                self.http_response = text;
                for char in self.http_response.chars() {
                    self.http_response_editor
                        .perform(text_editor::Action::Edit(text_editor::Edit::Insert(char)));
                }
                Task::none()
            }
            Message::UrlInput(text) => {
                self.url = text;
                Task::none()
            }
            Message::HttpResponseAction(action) => {
                self.http_response_editor.perform(action);
                Task::none()
            }
            Message::BodyInput(action) => {
                self.body_input_content.perform(action);
                Task::none()
            }
        }
    }
}
