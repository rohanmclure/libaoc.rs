use std::alloc::{Allocator, System, Layout};
use std::ops::{Index, IndexMut};
use std::ptr::NonNull;

use crate::matrix::view::MatrixView;

pub struct Matrix<T> {
    dims: (usize, usize),
    layout: Layout,
    base_ptr: NonNull<T>
}

impl<T: Copy> Matrix<T> {
    pub fn new<Idx>(m: Idx, n: Idx) -> Self
    where Idx: Into<usize> {
        let (m, n) = (m.into(), n.into());
        let layout = Layout::array::<T>(m*n).unwrap();
        Matrix { dims: (m, n),
                 layout: layout,
                 base_ptr: System.allocate(layout)
                                 .unwrap().cast::<T>() }
    }

    pub fn fill(&mut self, e: T) {
        let (m, n) = self.dims;
        for j in 0..n {
            for i in 0..m {
                self[(i, j)] = e;
            }
        }
    }
}

impl<T> Drop for Matrix<T> {
    fn drop(&mut self) {
        unsafe {
            System.deallocate(self.base_ptr.cast::<u8>(), self.layout);
        }
    }
}

impl<T, Idx> Index<(Idx, Idx)> for Matrix<T>
where
    Idx: Into<usize>
{
    type Output = T;

    fn index(&self, index: (Idx, Idx)) -> &Self::Output {
        let (i, j) = (index.0.into(), index.1.into());
        let (m, _) = self.dims;
        unsafe {
            self.base_ptr.as_ptr().add(i + j*m).as_ref()
        }.unwrap()
    }
}

impl <T, Idx> IndexMut<(Idx, Idx)> for Matrix<T>
where
    Idx: Into<usize>
{
    fn index_mut(&mut self, index: (Idx, Idx)) -> &mut Self::Output {
        let (i, j) = (index.0.into(), index.1.into());
        let (m, _) = self.dims;
        unsafe {
            self.base_ptr.as_ptr().add(i + j*m).as_mut()
        }.unwrap()
    }
}

impl<T> MatrixView<T> for Matrix<T> {
    fn get_dims(self) -> (usize, usize) {
        self.dims
    }
}
