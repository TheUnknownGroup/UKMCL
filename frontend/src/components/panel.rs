use freya::{prelude::*, router::RouterContext};
use utils::C;

use crate::routes::Pages;

pub fn panel() -> impl IntoElement {
    let ukmcl_icon: ImageSource = ("ukmcl_icon", include_bytes!("../../assets/icons/icon.png")).into();
    let home_icon: ImageSource = ("home_icon", include_bytes!("../../assets/icons/home.svg")).into();
    let instance_icon: ImageSource = ("instance_icon", include_bytes!("../../assets/icons/instances.svg")).into();
    let store_icon: ImageSource = ("store_icon", include_bytes!("../../assets/icons/store.svg")).into();
    let settings_icon: ImageSource = ("settings_icon", include_bytes!("../../assets/icons/settings.svg")).into();
    rect()
        .vertical()
        .background(C::TB.value())
        .height(Size::Fill)
        .width(Size::px(50.))
        .main_align(Alignment::SpaceBetween)
        .corner_radius(CornerRadius::new(8., 0., 0., 8.))
        .child(rect()
            .vertical()
            .child(
                rect()
                    .width(Size::Fill)
                    .height(Size::px(40.))
                    .child(ukmcl(ukmcl_icon))
            )
            .child(rect().height(Size::px(10.)))
            .child(home_btn(home_icon))
            .child(instances_btn(instance_icon))
            .child(store_btn(store_icon))
        )
        .child(settings_btn(settings_icon))
}

fn ukmcl(icon: ImageSource) -> impl IntoElement {
    Button::new()
        .background(Color::TRANSPARENT)
        .border_fill(Color::TRANSPARENT)
        .cursor(CursorIcon::Pointer)
        .hover_background(C::D.with_opacity(0.80))
        .rounded_none()
        .padding(5.)
        .corner_radius(CornerRadius::new(8., 0., 0., 0.))
        .on_press(|_| {
            if let Err(e) = open::that("https://github.com/TheUnknownGroup/UKMCL") {
                eprintln!("Failed to open website: {e}");
            }
        })
        .child(ImageViewer::new(icon).rounded_sm().height(Size::px(40.)).width(Size::px(40.)))
}

fn home_btn(icon: ImageSource) -> impl IntoElement {
    Button::new()
        .background(Color::TRANSPARENT)
        .border_fill(Color::TRANSPARENT)
        .cursor(CursorIcon::Pointer)
        .hover_background(C::D.with_opacity(0.80))
        .rounded_none()
        .child(SvgViewer::new(icon))
        .on_press(|_| {
            let _ = RouterContext::get().replace(Pages::Home);
        })
}

fn instances_btn(icon: ImageSource) -> impl IntoElement {
    Button::new()
        .background(Color::TRANSPARENT)
        .border_fill(Color::TRANSPARENT)
        .cursor(CursorIcon::Pointer)
        .hover_background(C::D.with_opacity(0.80))
        .rounded_none()
        .child(SvgViewer::new(icon))
        .on_press(|_| {
            let _ = RouterContext::get().replace(Pages::Instances);
        })
}

fn store_btn(icon: ImageSource) -> impl IntoElement {
    Button::new()
        .background(Color::TRANSPARENT)
        .border_fill(Color::TRANSPARENT)
        .cursor(CursorIcon::Pointer)
        .hover_background(C::D.with_opacity(0.80))
        .rounded_none()
        .child(SvgViewer::new(icon))
        .on_press(|_| {
            let _ = RouterContext::get().replace(Pages::Store);
        })
}

fn settings_btn(icon: ImageSource) -> impl IntoElement {
    Button::new()
        .background(Color::TRANSPARENT)
        .border_fill(Color::TRANSPARENT)
        .cursor(CursorIcon::Pointer)
        .hover_background(C::D.with_opacity(0.80))
        .rounded_none()
        .corner_radius(CornerRadius::new(0., 0., 0., 8.))
        .child(SvgViewer::new(icon).margin(Gaps::new_symmetric(4., 0.)))
        .on_press(|_| {
            let _ = RouterContext::get().replace(Pages::Settings);
        })
}