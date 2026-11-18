use iced::{widget::container, *};

use crate::{Message, Page, routes::{instances::instances, main_page::home, settings::settings, store::store}};

pub fn pages(page: Page, shown_text: String) -> Element<'static, Message> {
    let page: Element<'static, Message> = match page {
        Page::Home => home(shown_text),
        Page::Instances => instances(),
        Page::Store => store(),
        Page::Settings => settings(),
    };
    container(page).width(Fill).height(Fill).padding(4).into()
}