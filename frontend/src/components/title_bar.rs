use freya::{prelude::*, router::RouterContext};
use utils::C;

use crate::routes::Pages;

pub fn title_bar() -> impl IntoElement {
    let mini_icon: ImageSource = ("minimize_icon", include_bytes!("../../assets/icons/minimize.svg")).into();
    let full_screen: ImageSource = ("full_screen_icon", include_bytes!("../../assets/icons/fullscreen.svg")).into();
    let full_screen_exit: ImageSource = ("fullscreen_exit_icon", include_bytes!("../../assets/icons/fullscreen-exit.svg")).into();
    let close: ImageSource = ("close", include_bytes!("../../assets/icons/close.svg")).into();

    let maximized = use_state(|| false);

    let current_page: Pages = RouterContext::get().current();

    let title = match current_page {
        Pages::Home => "UKMCL",
        Pages::Instances => "UKMCL | Instances",
        Pages::Settings => "UKMCL | Settings",
        Pages::Store => "UKMCL | Store",
    };
    
    rect()
        .background(C::TB.value())
        .corner_radius(CornerRadius::new(8., 8., 0., 0.))
        .width(Size::Fill)
        .child(rect()
            .width(Size::Fill)
            .height(Size::Fill)
            .horizontal()
            .child(label()
                .text(title)
                .color(C::W.value())
                .text_align(TextAlign::Center).line_height(1.6)
            )
            .main_align(Alignment::SpaceBetween)
            .child(rect()
                .horizontal()
                .child(minimize_btn(mini_icon))
                .child(Button::new()
                    .height(Size::Fill)
                    .background(Color::TRANSPARENT)
                    .border_fill(Color::TRANSPARENT)
                    .cursor(Some(CursorIcon::Pointer))
                    .rounded_none()
                    .on_press({
                        let mut maximized = maximized.clone();
                        move |_| {
                            let next = !maximized();
                            Platform::get().with_window(Platform::window_id(), move |w| {
                                w.set_maximized(next);
                            });

                            maximized.set(next);
                        }
                    })
                    .child(SvgViewer::new(
                        if *maximized.read() {
                            full_screen_exit
                        } else {
                            full_screen
                        }
                    ))
                    .hover_background(C::W.with_opacity(0.10))
                )
                .child(close_btn(close))
            )
        )
        .height(Size::px(30.))
        .window_drag()
}

fn close_btn(close: ImageSource) -> impl IntoElement {
    Button::new()
        .child(SvgViewer::new(close))
        .height(Size::Fill)
        .background(Color::TRANSPARENT)
        .border_fill(Color::TRANSPARENT)
        .rounded_none()
        .cursor(Some(CursorIcon::Pointer))
        .corner_radius(CornerRadius::new(0., 8., 0., 0.))
        .on_press(|_| {
            Platform::get().close_window(Platform::window_id());
        })
        .hover_background(C::VC.with_opacity(0.50))
}

fn minimize_btn(mini_icon: ImageSource) -> impl IntoElement {
    Button::new()
        .child(SvgViewer::new(mini_icon))
        .height(Size::Fill)
        .background(Color::TRANSPARENT)
        .border_fill(Color::TRANSPARENT)
        .cursor(Some(CursorIcon::Pointer))
        .rounded_none()
        .on_press(|_| {
            Platform::get().with_window(Platform::window_id(), |w| {
                w.set_minimized(true);
            });
        })
        .hover_background(C::W.with_opacity(0.10))
}