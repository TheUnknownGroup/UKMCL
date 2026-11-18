use iced::{Theme, widget::text::Style};

use crate::style::UKMCLColors;

pub fn create_btn_instance(_: &Theme) -> Style {
    Style {
        color: Some(UKMCLColors::White.value()),
    }
}