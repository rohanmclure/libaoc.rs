use std::ops::{Index, IndexMut};

pub trait MatrixView<T>: Index<(usize, usize), Output = T> + IndexMut<(usize, usize)>
{
    fn get_dims(&self) -> (usize, usize);

    // fn view<Idx>(self, range_i: (Idx, Idx),
    //              range_j: (Idx, Idx)) -> SubMatrix<T>;
    fn view(self, range_i: (usize, usize),
            range_j: (usize, usize)) -> SubMatrix<T>;
    fn transpose(self) -> Transpose<T>;
}

/*
 * Support for views into larger matrices
 */
pub struct SubMatrix<T> {
    backing: Box<dyn MatrixView<T>>,
    range_i: (usize, usize),
    range_j: (usize, usize)
}

impl<T, Idx> Index<(Idx, Idx)> for SubMatrix<T>
where
    Idx: Into<usize>
{
    type Output = T;

    fn index(&self, index: (Idx, Idx)) -> &Self::Output {
        let (ri, rj) = (self.range_i.0,
                        self.range_j.0);
        let (i, j): (usize, usize) = (index.0.into(), index.1.into());
        &(*self.backing)[(ri + i, rj + j)]
    }
}

impl<T, Idx> IndexMut<(Idx, Idx)> for SubMatrix<T>
where
    Idx: Into<usize>
{
    fn index_mut(&mut self, index: (Idx, Idx)) -> &mut Self::Output {
        let (ri, rj) = (self.range_i.0,
                        self.range_j.0);
        let (i, j): (usize, usize) = (index.0.into(), index.1.into());
        &mut (*self.backing)[(ri + i, rj + j)]
    }
}

impl<T> MatrixView<T> for SubMatrix<T> {
    fn get_dims(&self) -> (usize, usize) {
        (self.range_i.1 - self.range_i.0 + 1,
         self.range_j.1 - self.range_j.0 + 1)
    }

    fn view(self, range_i: (usize, usize),
            range_j: (usize, usize)) -> SubMatrix<T> {
        SubMatrix { backing: Box::new(self), range_i, range_j }
    }

    fn transpose(self) -> Transpose<T> {
        Transpose { backing: Box::new(self) }
    }
}

/*
 * Lazy transpose into a submatrix
 */
pub struct Transpose<T> {
    backing: Box<dyn MatrixView<T>>
}
