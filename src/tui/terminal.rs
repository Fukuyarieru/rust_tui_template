use crate::tui::selection_box::SelectionBox;

pub struct Terminal {
    // display: Display
    regions: Vec<Box<dyn SelectionBox>>, // Box<> is temporary
}
