use iced::{widget::{button, container, mouse_area, row, text}, window::Id, *};

use crate::{Message, Page, WinMsg, WindowKind, style::*};

pub fn title_bar(id: Id, max: bool, p: Page, win_kind: WindowKind) -> Element<'static, Message> {
    let l = 20.0;

    let m = match max {
        true => fullexit_i::<Message>(IconSize { w: l, h: l}),
        false => full_i::<Message>(IconSize { w: l, h: l}),
    };

    let title = match win_kind {
        WindowKind::CreateInstance => "UKMCL | Create an Instance",
        WindowKind::Main => match p {
            Page::Home => "UKMCL",
            Page::Instances => "UKMCL | Instances",
            Page::Store => "UKMCL | Store",
            Page::Settings => "UKMCL | Settings",
        }
    };
    
    container(
        mouse_area(
            row![
                container(text("")).width(120),
                text(title).color(Color::WHITE).width(Fill).align_x(Center).size(16),
                button(mini_i::<Message>(IconSize { w: l, h: l})).padding([5, 10]).on_press(Message::TitleBar(id, WinMsg::Minimize)).style(mini_max_btn),
                button(m).padding([5, 10]).on_press(Message::TitleBar(id, WinMsg::Maximize)).style(mini_max_btn),
                button(close_i::<Message>(IconSize { w: l, h: l})).padding([5, 10]).on_press(Message::TitleBar(id, WinMsg::Close)).style(close_btn),
            ].align_y(Center).width(Fill)
        ).on_press(Message::TitleBar(id, WinMsg::DragWindow))
    )
    .style(titlebar_style)
    .into()
}