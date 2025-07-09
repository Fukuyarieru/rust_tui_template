pub struct Grid<T> {
    vec: Vec<Vec<T>>,
    width: usize,
    height: usize,
}
impl<T: Default + Clone> Grid<T> {
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
    pub(crate) fn from(vecs: Vec<Vec<T>>) -> Self {
        Grid { vec: vecs.clone(), width: vecs.len(), height: vecs[0].len() }
    }
}

impl<T> IntoIterator for Grid<T> {
    type Item=T;

    type IntoIter = std::iter::Flatten<std::vec::IntoIter<Vec<T>>>;
    fn into_iter(self) -> Self::IntoIter {
        self.vec.into_iter().flatten()
    }
}
impl<T: Default+Clone> From<Vec<Vec<T>>> for Grid<T> {
    fn from(value: Vec<Vec<T>>) -> Self {
        Grid::from(value)
    }
}
// impl<T> Into<Grid<T>> for Iter<'_,Iter<'_,T>> {
//     fn into(self) -> Grid<T> {
//         self.map(|iter: &Iter<'_, T>|iter.)
//     }
// }