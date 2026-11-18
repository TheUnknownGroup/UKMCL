use iced::{widget::{container, row}, *};

use crate::Message;

pub struct CreateInstance {
    name: String,
    versions: Vec<String>,
    loader_versions: Vec<String>,
    selected_version: String,
    selected_loader: String,
    current_loader: String,
    loading: bool,
    create: bool,
}

pub fn create_instance_window_code() -> Element<'static, Message> {
    container(row![]).into()
}