use std::alloc::{Allocator, System, Layout};
use std::fmt::Debug;
use std::ops::{Index, IndexMut};
use std::ptr::NonNull;

pub struct Matrix<T> {
    dims: (usize, usize),
    layout: Layout,
    base_ptr: NonNull<T>
}

impl<T> Matrix<T> {
    pub fn new(m: usize, n: usize) -> Self {
        let layout = Layout::array::<T>(m*n).unwrap();
        Matrix { dims: (m, n),
                 layout,
                 base_ptr: System.allocate(layout)
                                 .unwrap().cast::<T>()
        }
    }

    pub fn get_dims(&self) -> (usize, usize) {
        self.dims
    }
}

impl<T: Copy> Matrix<T> {
    pub fn fill(mut self, e: T) -> Self {
        let (m, n) = self.dims;
        for j in 0..n {
            for i in 0..m {
                self[(i, j)] = e;
            }
        }
        self
    }
    
    pub fn fill_with<Idx, E1: Debug, E2: Debug, F>(mut self, f: F) -> Self
    where
        Idx: TryInto<isize, Error = E1> + TryFrom<usize, Error = E2>,
        F: Fn((Idx, Idx)) -> T
    {
        let (m, n) = self.dims;
        for j in 0..n {
            for i in 0..m {
                self[(i, j)] = f((i.try_into().unwrap(),
                                  j.try_into().unwrap()));
            }
        }
        self
    }
}

impl<T> Drop for Matrix<T> {
    fn drop(&mut self) {
        unsafe {
            System.deallocate(self.base_ptr.cast::<u8>(), self.layout);
        }
    }
}

impl<T, Idx, E: Debug> Index<(Idx, Idx)> for Matrix<T>
where
    Idx: TryInto<isize, Error = E>
{
    type Output = T;

    fn index(&self, index: (Idx, Idx)) -> &Self::Output {
        let (i, j): (isize, isize) = (index.0.try_into().unwrap(),
                                      index.1.try_into().unwrap());
        let (i, j): (usize, usize) = (i.try_into().unwrap(),
                                      j.try_into().unwrap());
        let (m, n) = self.dims;
        assert!(i < m && j < n, "Index out of bounds at ({i},{j})");
        unsafe {
            self.base_ptr.as_ptr().add(i + j*m).as_ref()
        }.unwrap()
    }
}

impl <T, Idx, E: Debug> IndexMut<(Idx, Idx)> for Matrix<T>
where
    Idx: TryInto<isize, Error = E>
{
    fn index_mut(&mut self, index: (Idx, Idx)) -> &mut Self::Output {
        let (i, j): (isize, isize) = (index.0.try_into().unwrap(),
                                      index.1.try_into().unwrap());
        let (i, j): (usize, usize) = (i.try_into().unwrap(),
                                      j.try_into().unwrap());
        let m = self.dims.0;
        unsafe {
            self.base_ptr.as_ptr().add((i + j*m).try_into().unwrap()).as_mut()
        }.unwrap()
    }
}
