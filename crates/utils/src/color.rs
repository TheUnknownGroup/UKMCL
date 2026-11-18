use hex_rgb_converter::{Color};

pub fn to_rgb(hex: &str) -> iced::Color {
    let rgb = Color::hex(hex).to_rgb();
    iced::Color::from_rgb8(rgb.r, rgb.g, rgb.b)
}

pub fn to_rgba(hex: &str, o: f32) -> iced::Color {
    let rgb = Color::hex(hex).to_rgb();
    iced::Color::from_rgba8(rgb.r, rgb.g, rgb.b, o)
}