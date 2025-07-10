#[doc(inline)]


use std::sync::Arc;

use crate::prelude::*;

/// EXAMPLE
/// ``````
///  pub struct Opus {
///     pub vec: Vec<Arc<Pixel>>
///  }
///
///  impl FromIterator<Arc<Pixel>> for Opus {
///      fn from_iter<T: IntoIterator<Item = Arc<Pixel>>>(iter: T) -> Self {
///          Opus { vec: iter.into_iter().collect() }
///     }
/// }
///
///  impl DisplayObject for Opus {
///      fn get_data(&self)->impl Iterator {
///          self.vec.iter()
///      }
///  }

pub trait DisplayObject : FromIterator<Arc<Pixel>> {
    fn get_data(&self)->impl Iterator;
}

pub trait SelectionBox : DisplayObject {
    fn get_data(&self) -> Grid<Arc<Pixel>>;
    fn center(&self) -> (usize,usize);
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}
