use crate::prelude::*;

pub struct Terminal {
    // display: Display
    regions: Vec<impl SelectionBox>, // Box<> is temporary
}
