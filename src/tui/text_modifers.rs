pub struct TextModifier {
    italic: bool,
    bold: bool,
}

impl Default for TextModifier {
    fn default() -> Self {
        Self { italic: false, bold: false }
    }
}