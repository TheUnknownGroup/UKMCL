use iced::{Background, Border, Theme, border::Radius, widget::container::*};

use crate::style::color::UKMCLColors;

pub fn titlebar_style(_: &Theme) -> Style {
    Style {
        background: Some(Background::Color(UKMCLColors::TitleBar.value())),
        border: Border {
            radius: Radius {
                top_left: 12.0,
                top_right: 12.0,
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn panel_style(_: &Theme) -> Style {
    Style {
        background: Some(Background::Color(UKMCLColors::TitleBar.value())),
        border: Border {
            radius: Radius {
                top_left: 12.0,
                bottom_left: 12.0,
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn welcome_style(_: &Theme) -> Style {
    Style {
        background: Some(Background::Color(UKMCLColors::AB950.with_opacity(0.80))),
        border: Border {
            radius: 4.0.into(),
            ..Default::default()
        },
        ..Default::default()
    }
}