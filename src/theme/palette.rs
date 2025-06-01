use bevy::prelude::*;

const fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::srgb(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
}

pub const CLEAR_COLOR: Color = rgb(50,64,76);
pub const LABEL_TEXT: Color = rgb(145,248,244);
pub const HEADER_TEXT: Color = rgb(255,239,158);
pub const BUTTON_TEXT: Color = rgb(63,23,126);
pub const BUTTON_BACKGROUND: Color = rgb(203,190,249);
pub const BUTTON_HOVERED_BACKGROUND: Color = rgb(245,161,247);
pub const BUTTON_PRESSED_BACKGROUND: Color = rgb(186,154,245);
