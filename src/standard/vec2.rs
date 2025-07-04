pub struct Vec2<T> {
    vec: Vec<Vec<T>>,
    width: usize,
    height: usize,
}
impl<T> Vec2<T> {
    pub fn new(width: usize, height: usize) -> Self
    where
        T: Default + Copy,
    {
        Self {
            vec: {
                // TODO
                // let vec: Vec<Vec<T>> =
                //     Vec::<Vec<T>>::with_capacity(width).fill(Vec::with_capacity(height));
                Vec::new()
            },
            width: width,
            height: height,
        }
    }
}
