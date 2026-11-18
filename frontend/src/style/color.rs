use iced::Color;
use utils::*;

pub enum UKMCLColors {
    BG,
    TitleBar,
    White,
    VanillaCustard,
    Dark,
    AB950
}

impl UKMCLColors {
    pub fn value(&self) -> Color {
        match self {
            Self::AB950 => to_rgb("09131a"),
            Self::BG => to_rgb("1B364B"),
            Self::TitleBar => to_rgb("0B2A3C"),
            Self::VanillaCustard => to_rgb("CFCD96"),
            Self::Dark => to_rgb("0E1B25"),
            Self::White => to_rgb("FFFFFF"),
        }
    }
    pub fn with_opacity(&self, o: f32) -> Color {
        match self {
            Self::AB950 => to_rgba("09131a", o),
            Self::BG => to_rgba("1B364B", o),
            Self::TitleBar => to_rgba("0B2A3C", o),
            Self::VanillaCustard => to_rgba("CFCD96", o),
            Self::Dark => to_rgba("0E1B25", o),
            Self::White => to_rgba("FFFFFF", o),
        }
    }
}