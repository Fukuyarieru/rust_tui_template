use std::sync::Arc;

use crate::prelude::*;

// name is temporary

pub trait DisplayObject {
    fn get_data(&self)->Vec<Arc<Pixel>>;
}
impl<T:Iterator,G: DisplayObject> From<T> for G {
    fn from(value: T) -> Self {
        todo!()
    }
}

pub trait SelectionBox : DisplayObject {
    fn get_data(&self) -> Grid<Arc<Pixel>>;
    fn center(&self) -> (usize,usize);
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}
