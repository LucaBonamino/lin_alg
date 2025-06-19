use std::ops::BitXor;

trait UnsignedInteger:
Copy + Eq + Ord + BitXor<Self> 
{}

pub trait MatrixTraid<T>{
    fn rank(&self)-> usize;
    fn kernel(&self) -> Vec<Vec<T>>;
    fn echelon_form(&self) -> Self;
}

struct Matrix<T: UnsignedInteger>{
    elements: Vec<Vec<T>>
}