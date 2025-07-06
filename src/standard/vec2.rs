pub struct Vec2<T> {
    vec: Vec<Vec<T>>,
    width: usize,
    height: usize,
}
impl<T: Default + Clone> Vec2<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            vec: vec![vec![T::default(); height]; width],
            width: width,
            height: height,
        }
    }
    pub fn get(&self, x: usize, y: usize) -> Option<T> {
        if let Some(vec) = self.vec.get(x) {
            vec.get(y).cloned()
        } else {
            None
        }
    }
    pub fn set(&mut self, x: usize, y: usize, new_val: T) -> Result<(), ()> {
        if x < self.width && x >= 0 && y < self.height && y >= 0 {
            self.vec[x][y] = new_val;
            return Ok(());
        } else {
            Err(())
        }
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
}
