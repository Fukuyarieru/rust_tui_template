use crate::prelude::*;

pub struct Terminal {
    // display: Display
    display: Display,
    regions: Vec<Box<dyn SelectionBox>>, // Box<> is temporary
}
