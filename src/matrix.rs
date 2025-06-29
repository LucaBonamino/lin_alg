use std::ops::BitXor;
use num_traits::Zero;

pub trait Number:
Copy + Eq + Ord + BitXor<Self> + Zero
{}

impl Number for u8 {}

pub trait MatrixTrait<T: Number>: HasElements<T>{
    fn rank(&self)-> usize;
    fn kernel(&self) -> Vec<Vec<T>>;
    fn echelon_form(&self) -> Self;
    fn image(&self) -> Vec<Vec<T>>;
    fn ncols(&self) -> usize {
         self.elements().len()
    }
    fn nrows(&self) -> usize {
        self.elements().get(0).map_or(0, |row| row.len())
    }
    fn get_pivot(vec: &Vec<T>) -> Option<usize> {
         vec.iter().position(|&x| !x.is_zero())
    }
}

pub trait HasElements<T: Number> {
    fn elements(&self) -> &Vec<Vec<T>>;
}

pub struct Matrix<T: Number>{
    pub elements: Vec<Vec<T>>
}

impl<T: Number> Matrix<T> {
    fn new(elements: Vec<Vec<T>>) -> Self{
        Self { elements}
    }
}

impl <T: Number> HasElements<T> for Matrix<T> {
    fn elements(&self) -> &Vec<Vec<T>> {
        &self.elements
    }
}