use freya::prelude::*;

pub enum C {
    BG,
    TB,
    W,
    VC,
    D,
    AB950,
    C,
}

impl C {
    pub fn value(&self) -> Color {
        match self {
            Self::BG => Color::from_hex("1B364B").unwrap(),
            Self::TB => Color::from_hex("0B2A3C").unwrap(),
            Self::W => Color::from_hex("FFFFFF").unwrap(),
            Self::VC => Color::from_hex("CFCD96").unwrap(),
            Self::D => Color::from_hex("0E1B25").unwrap(),
            Self::AB950 => Color::from_hex("09131a").unwrap(),
            Self::C => Color::from_hex("102313").unwrap(),
        }
    }

    pub fn with_opacity(&self, o: f32) -> Color {
        match self {
            Self::BG => from_hex_to_rgba("1B364B", o),
            Self::TB => from_hex_to_rgba("0B2A3C", o),
            Self::W => from_hex_to_rgba("FFFFFF", o),
            Self::VC => from_hex_to_rgba("CFCD96", o),
            Self::D => from_hex_to_rgba("0E1B25", o),
            Self::AB950 => from_hex_to_rgba("09131a", o),
            Self::C => from_hex_to_rgba("102313", o),
        }
    }
}

fn from_hex_to_rgba(hex: &str, o: f32) -> Color {
    let hex_decode = Color::from_hex(hex).unwrap();
    let r = hex_decode.r();
    let g = hex_decode.g();
    let b = hex_decode.b();
    Color::from_argb((o.clamp(0.0, 1.0) * 255.0) as u8, r, g, b)
}
