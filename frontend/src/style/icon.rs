use iced::{*, widget::svg};

pub struct IconSize {
    pub w: f32,
    pub h: f32,
}

// Title Bar Icons
pub fn full_i<Message: 'static>(s: IconSize) -> Element<'static, Message> {
    svg(svg::Handle::from_memory(include_bytes!("../../assets/icons/fullscreen.svg").as_slice())).width(s.w).height(s.h).into()
}

pub fn fullexit_i<Message: 'static>(s: IconSize) -> Element<'static, Message> {
    svg(svg::Handle::from_memory(include_bytes!("../../assets/icons/fullscreen-exit.svg").as_slice())).width(s.w).height(s.h).into()
}

pub fn close_i<Message: 'static>(s: IconSize) -> Element<'static, Message> {
    svg(svg::Handle::from_memory(include_bytes!("../../assets/icons/close.svg").as_slice())).width(s.w).height(s.h).into()
}

pub fn mini_i<Message: 'static>(s: IconSize) -> Element<'static, Message> {
    svg(svg::Handle::from_memory(include_bytes!("../../assets/icons/minimize.svg").as_slice())).width(s.w).height(s.h).into()
}

// Panel Icons
pub fn home_i<Message: 'static>(s: IconSize) -> Element<'static, Message> {
    svg(svg::Handle::from_memory(include_bytes!("../../assets/icons/home.svg").as_slice())).width(s.w).height(s.h).into()
}

pub fn inst_i<Message: 'static>(s: IconSize) -> Element<'static, Message> {
    svg(svg::Handle::from_memory(include_bytes!("../../assets/icons/instances.svg").as_slice())).width(s.w).height(s.h).into()
}

pub fn store_i<Message: 'static>(s: IconSize) -> Element<'static, Message> {
    svg(svg::Handle::from_memory(include_bytes!("../../assets/icons/store.svg").as_slice())).width(s.w).height(s.h).into()
}

pub fn setts_i<Message: 'static>(s: IconSize) -> Element<'static, Message> {
    svg(svg::Handle::from_memory(include_bytes!("../../assets/icons/settings.svg").as_slice())).width(s.w).height(s.h).into()
}

pub fn ukmcl_i<Message: 'static>(s: IconSize) -> Element<'static, Message> {
    svg(svg::Handle::from_memory(include_bytes!("../../assets/icons/icon.svg").as_slice())).width(s.w).height(s.h).into()
}

// Create Button
pub fn create_btn_i<Message: 'static>(s: IconSize) -> Element<'static, Message> {
    svg(svg::Handle::from_memory(include_bytes!("../../assets/icons/add.svg").as_slice())).width(s.w).height(s.h).into()
}