use iced::{widget::{container, row, text}, *};

use crate::{Message, style::*};

pub fn home(shown: String) -> Element<'static, Message> {
    container(
        row![
            container(text(shown).size(23)).align_x(Center).style(welcome_style).padding(Padding {
                top: 5.0,
                bottom: 5.0,
                left: 5.0,
                right: 50.0,
            }).width(Fill)
        ].width(Fill)
    ).into()
}