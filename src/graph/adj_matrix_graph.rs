use std::collections::HashMap;
use std::hash::Hash;


use super::super::matrix::matrix::Matrix;
use super::graph::Graph;

// struct MatrixDigraph<V, E> {
//     vmap: Vec<V>,
//     adj: Matrix<E>
// }

struct MatrixGraph<V, E> {
    vmap: Vec<V>,
    hmap: HashMap<V, usize>,
    adj: Matrix<Option<E>>
}

impl<V, E> MatrixGraph<V, E>
where
    V: Copy+Eq+Hash
{
    pub fn new(vmap: Box<dyn Iterator<Item = &V>>,
               adj: Matrix<Option<E>>)
        -> Self
    {
        let mut hmap = HashMap::new();
        let vmap = vmap.enumerate().map(&mut |(i, &v)| {
            hmap.insert(v, i);
            v
        }).collect();

        MatrixGraph {
            vmap,
            hmap,
            adj
        }
    }

    fn get_idx(&self, v: &V) -> usize {
        self.hmap[v]
    }
}

impl<'a, V: 'a, E: 'a> Graph<'a, V, E> for MatrixGraph<V, E>
where
    V: Copy+Eq+Hash,
    E: Copy
{
    fn get_neighbours(&'a self, v: &'a V)
                   -> impl Iterator<Item=&'a V> {
        let i = self.get_idx(v);
        let mut n = Vec::new();
        for j in 0 .. self.adj.get_dims().1 {
            if let Some(_) = self.adj[(i,j)] {
                n.push(&self.vmap[j]);
            }
        }
        n.into_iter()
    }

    fn get_arc(&self, v: &V, u: &V) -> Option<E> {
        let idx = (self.get_idx(v), self.get_idx(u));
        self.adj[idx]
    }
}
