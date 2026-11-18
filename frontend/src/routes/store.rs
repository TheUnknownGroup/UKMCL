use iced::{widget::row, *};

use crate::Message;

pub fn store() -> Element<'static, Message> {
    row![].into()
}