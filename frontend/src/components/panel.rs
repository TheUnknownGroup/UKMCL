use iced::{widget::{Space, button, column, container}, *};

use crate::{Message, Page, style::*};

pub fn panel() -> Element<'static, Message> {
    let l = 28.0;
    container(
        column![
            button(ukmcl_i::<Message>(IconSize { w: 0.0, h: 40.0 })).width(50).style(icon_btn).padding([5, 0]),
            button(home_i::<Message>(IconSize { w: l, h: l})).width(50).on_press(Message::Nav(Page::Home)).style(panel_btn).padding([7, 0]),
            button(inst_i::<Message>(IconSize { w: l, h: l})).width(50).on_press(Message::Nav(Page::Instances)).style(panel_btn).padding([7, 0]),
            button(store_i::<Message>(IconSize { w: l, h: l})).width(50).on_press(Message::Nav(Page::Store)).style(panel_btn).padding([7, 0]),
            Space::new().height(Fill),
            button(setts_i::<Message>(IconSize { w: l, h: l})).width(50).on_press(Message::Nav(Page::Settings)).style(sett_btn).padding(Padding { bottom: 8.0, top: 8.0, ..Default::default()}),
        ].align_x(Center).height(Fill),
    ).style(panel_style).height(Fill).align_x(Center).into()
}