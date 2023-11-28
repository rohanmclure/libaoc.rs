pub mod matrix;
pub mod view;

#[cfg(test)]
#[test]
fn matrix_fill() {
    let m = matrix::Matrix::new(10, 20).fill(0.0);
    for i in 0 .. 10 {
        for j in 0 .. 20 {
            assert_eq!(m[(i,j)], 0.0);
        }
    }
}

#[test]
fn matrix_transpose() {
    let mut m = matrix::Matrix::new(10, 20); 

    for i in 0 .. 10 {
        for j in 0 .. 20 {
            m[(i,j)] = (i,j);
        }
    }

    let t = m.transpose();
    assert_eq!(t.get_dims(), (20, 10));
    for i in 0 .. 20 {
        for j in 0 .. 10 {
            assert_eq!(t[(i,j)], (j,i));
        }
    }
}

#[test]
fn matrix_view() {
    let mut m = matrix::Matrix::new(10, 20);

    for i in 0 .. 10 {
        for j in 0 .. 20 {
            m[(i,j)] = (i,j);
        }
    }

    let v = m.view((5,10), (10,20));
    for i in 5 .. 10 {
        for j in 10 .. 20 {
            assert_eq!(v[(i,j)], (i,j));
        }
    }
}

#[test]
fn matrix_mixed() {
    let m = matrix::Matrix::new(10, 20).fill_with(|idx: (isize, isize)| idx);

    for i in 0 .. 10 {
        for j in 0 .. 20 {
            assert_eq!(m[(i,j)], (i,j));
        }
    }

    let vt = m.view((5,10), (10,20))
              .transpose();
    for i in 5 .. 10 {
        for j in 10 .. 20 {
            assert_eq!(vt[(i,j)], (j,i));
        }
    }
}
