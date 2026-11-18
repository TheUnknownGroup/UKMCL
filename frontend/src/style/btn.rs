use iced::{Background, Border, Color, Shadow, Theme, Vector, border::Radius, widget::button::*};

use crate::style::color::UKMCLColors;

pub fn mini_max_btn(_: &Theme, status: Status) -> Style {
    let back = match status {
        Status::Active => Color::TRANSPARENT,
        Status::Hovered => UKMCLColors::White.with_opacity(0.10),
        Status::Disabled => Color::TRANSPARENT,
        Status::Pressed => UKMCLColors::White.with_opacity(0.10),
    };
    
    Style {
        background: Some(Background::Color(back)),
        text_color: Color::WHITE,
        ..Default::default()
    }
}

pub fn close_btn(_: &Theme, status: Status) -> Style {
    let back = match status {
        Status::Active => Color::TRANSPARENT,
        Status::Hovered => UKMCLColors::VanillaCustard.with_opacity(0.50),
        Status::Disabled => Color::TRANSPARENT,
        Status::Pressed => UKMCLColors::VanillaCustard.with_opacity(0.50),
    };

    Style {
        background: Some(Background::Color(back)),
        text_color: Color::WHITE,
        border: Border {
            radius: Radius {
                top_right: 12.0,
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn panel_btn(_: &Theme, status: Status) -> Style {
    let back = match status {
        Status::Active => Color::TRANSPARENT,
        Status::Hovered => UKMCLColors::Dark.with_opacity(0.80),
        Status::Disabled => Color::TRANSPARENT,
        Status::Pressed => UKMCLColors::Dark.with_opacity(0.80),
    };
    
    Style {
        background: Some(Background::Color(back)),
        ..Default::default()
    }
}

pub fn sett_btn(_: &Theme, status: Status) -> Style {
    let back = match status {
        Status::Active => Color::TRANSPARENT,
        Status::Hovered => UKMCLColors::Dark.with_opacity(0.80),
        Status::Disabled => Color::TRANSPARENT,
        Status::Pressed => UKMCLColors::Dark.with_opacity(0.80),
    };
    
    Style {
        background: Some(Background::Color(back)),
        border: Border {
            radius: Radius {
                bottom_left: 12.0,
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn icon_btn(_: &Theme, status: Status) -> Style {
    let back = match status {
        Status::Active => Color::TRANSPARENT,
        Status::Hovered => UKMCLColors::Dark.with_opacity(0.80),
        Status::Disabled => Color::TRANSPARENT,
        Status::Pressed => UKMCLColors::Dark.with_opacity(0.80),
    };
    
    let shadow = match status {
        Status::Active => Shadow::default(),
        Status::Hovered => Shadow { 
            color: UKMCLColors::VanillaCustard.with_opacity(0.80),
            offset: Vector { x: 0.0, y: 0.0 },
            blur_radius: 5.0,
        },
        Status::Disabled => Shadow::default(),
        Status::Pressed => Shadow { 
            color: UKMCLColors::VanillaCustard.with_opacity(0.80),
            offset: Vector { x: 0.0, y: 0.0 },
            blur_radius: 5.0,
        },
    };
    
    Style {
        background: Some(Background::Color(back)),
        shadow,
        ..Default::default()
    }
}

pub fn create_btn_instance_list(_: &Theme, _: Status) -> Style {
    Style {
        background: Some(Background::Color(UKMCLColors::Dark.with_opacity(0.90))),
        border: Border {
            color: Color::BLACK,
            width: 1.5,
            radius: 5.0.into(),
        },
        ..Default::default()
    }
}