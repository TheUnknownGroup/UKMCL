use iced::{widget::{button, row}, *};
use crate::{Message, PossibleWindows, style::*};

pub fn instances() -> Element<'static, Message> {
    row![
        button(
            create_btn_i(IconSize { w: 20.0, h: 20.0 })
        ).style(create_btn_instance_list).width(200.0).on_press(Message::Windows(PossibleWindows::OpenCreateWindow)).padding([10, 0])
    ].into()
}