use iced::{border::Radius, widget::{Space, column, container, row, stack}, window::Id, *};
use crate::{Message, Page, Window, WindowKind, components::{create_inst::create_instance_window_code, panel::panel, title_bar::title_bar}, routes::pages::pages, style::color::UKMCLColors};

pub fn app<'a>(id: Id, win: &'a Window, page: Page, shown_text: String) -> Element<'a, Message> {
    match win.kind {
        WindowKind::Main => {
            container(stack![
                column![
                    title_bar(id, win.is_maxed, page, win.kind.clone()),
                    row![
                        Space::new().width(50),
                        container(pages(page, shown_text))
                            .width(Fill)
                            .height(Fill).style(|_| container::Style {
                                background: Some(Background::Color(UKMCLColors::BG.value())),
                                border: Border {
                                    radius: Radius {
                                        top_left: 8.0,
                                        bottom_right: if win.is_maxed { 0.0 } else { 12.0 },
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                },
                                ..Default::default()
                            }),
                    ].height(Fill)
                ].width(Fill).height(Fill),
                panel()
            ]).into()
        }
        WindowKind::CreateInstance => {
            stack![
                column![
                    title_bar(id, win.is_maxed, page, win.kind.clone()),
                    row![
                        container(create_instance_window_code()).width(Fill).height(Fill).style(|_| container::Style {
                            background: Some(Background::Color(UKMCLColors::BG.value())),
                            border: Border {
                                radius: Radius {
                                    bottom_left: if win.is_maxed { 0.0 } else { 12.0 },
                                    bottom_right: if win.is_maxed { 0.0 } else { 12.0 },
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                    ]
                ]
            ].into()
        }
    }
}