use std::sync::Arc;

use crate::prelude::*;

pub struct Display {
    grid: Grid<Arc<Pixel>>,
}

impl Display {
    pub fn draw_line(p1:(usize,usize),p2:(usize,usize)) {}
    pub fn get_pixel(&self,x:usize,y:usize) -> Option<Arc<Pixel>> {
        self.grid.get(x, y)
    }
    pub fn get_region(&self,center: (usize,usize),width:usize,height:usize) -> Grid<Arc<Pixel>> {
        let left= if center.0< width {0} else {center.0-width};
        let up=if center.1+height > self.grid.height() {self.grid.height()} else {center.1+height};
        let right;
        let down;

        let width=left-right;
        let height=up-down;

        let mut grid=Grid::new(width, height);

        for width_index in left..right {
            for height_index in up..down {
                grid.set(width_index-right,height_index-up,self.get_pixel(width_index, height_index));
            }
        }
        let a=grid.into_iter().map(|optional_arc_pixel|optional_arc_pixel.unwrap()).into();
        a
    }
}

