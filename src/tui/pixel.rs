use crate::prelude::*;

#[derive(Default)]
pub struct Pixel {
    char: char,
    background_color: Color,
    foreground_color: Color,
    text_modifers: TextModifier,
}
