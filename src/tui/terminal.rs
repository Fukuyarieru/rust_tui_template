use crate::prelude::*;

pub struct Terminal {
    // display: Display
    pub display: Display,
    regions: Vec<Box<dyn SelectionBox>>, // Box<> is temporary
}

impl Terminal {
    pub fn render(&self) {}
}