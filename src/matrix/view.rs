use std::{ops::{Index, IndexMut}, fmt::Debug};

use super::matrix::Matrix;

enum Backing<'a, T> {
    Real(&'a Matrix<T>),
    View(Box::<MView<'a, T>>)
}

pub struct MView<'a, T> {
    map: Box<dyn IndexMap>,
    backing: Backing<'a, T>
}

impl<'a, T> MView<'a, T> {
    pub fn get_domain(&self) -> ((isize, isize), (isize, isize)) {
        (&*self.map).get_domain()
    }

    pub fn get_dims(&self) -> (usize, usize) {
        let (range_i, range_j) = self.get_domain();
        ((range_i.1 - range_i.0).try_into().unwrap(),
         (range_j.1 - range_j.0).try_into().unwrap())
    }
}

impl<'a, T, Idx, E: Debug> Index<(Idx, Idx)> for MView<'a, T>
where
    Idx: TryInto<isize, Error = E>
{
    type Output = T;

    fn index(&self, idx: (Idx, Idx)) -> &Self::Output {
        let idx = self.map.index_map((idx.0.try_into().unwrap(),
                                      idx.1.try_into().unwrap()));
        match &self.backing {
            &Backing::Real(m) => &m[idx],
            Backing::View(v)  => &v[idx]
        }
    }
}

/* Some default index maps */
pub trait IndexMap {
    fn get_domain(&self) -> ((isize, isize), (isize, isize));
    fn index_map(&self, idx: (isize, isize)) -> (isize, isize);
}


struct TransposeMap {
    range_i: (isize, isize),
    range_j: (isize, isize)
}

impl IndexMap for TransposeMap {
    fn get_domain(&self) -> ((isize, isize), (isize, isize)) {
        (self.range_i, self.range_j)
    }

    fn index_map(&self, idx: (isize, isize)) -> (isize, isize) {
        let (i, j) = idx;
        (j, i)
    }
}


pub struct OffsetMap {
    start_offset: (isize, isize),
    src_domain: ((isize, isize), (isize, isize))
}

impl IndexMap for OffsetMap {
    fn get_domain(&self) -> ((isize, isize), (isize, isize)) {
        let (dims_i, dims_j) = self.src_domain;
        let (si, sj) = self.start_offset;
        ((dims_i.0 - si, dims_i.1 - si),
         (dims_j.0 - sj, dims_j.1 - sj))
    }

    fn index_map(&self, idx: (isize, isize)) -> (isize, isize) {
        let (i, j) = idx;
        (i + self.start_offset.0,
         j + self.start_offset.1)
    }
}


/*
 * Factories for creating immutable views into the matrix.
 */

impl<T> Matrix<T> {
    pub fn view(&self, range_i: (isize, isize),
                range_j: (isize, isize)) -> MView<T> {
        let (m, n) = self.get_dims();
        MView {
            map: Box::new(OffsetMap {
                start_offset: (range_i.0,
                               range_j.0),
                src_domain: ((0, m.try_into().unwrap()),
                             (0, n.try_into().unwrap()))
            }),
            backing: Backing::Real(self)
        }
    }

    pub fn transpose(&self) -> MView<T> {
        let (m, n) = self.get_dims();
        MView {
            map: Box::new(TransposeMap {
                range_i: (0, n.try_into().unwrap()),
                range_j: (0, m.try_into().unwrap())
            }),
            backing: Backing::Real(self)
        }
    }
}

/*
 * The same factories for view types.
 */
impl<'a, T> MView<'a, T> {
    pub fn view(self, range_i: (isize, isize),
                range_j: (isize, isize)) -> MView<'a, T> {
        MView {
            map: Box::new(OffsetMap {
                start_offset: (range_i.0,
                               range_j.0),
                src_domain: self.get_domain()
            }),
            backing: Backing::View(Box::new(self))
        }
    }

    pub fn transpose(self) -> MView<'a, T> {
        let (m, n) = self.get_dims();
        MView {
            map: Box::new(TransposeMap {
                range_i: (0, n.try_into().unwrap()),
                range_j: (0, m.try_into().unwrap())
            }),
            backing: Backing::View(Box::new(self)),
        }
    }
}
