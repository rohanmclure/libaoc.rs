// pub trait ExtendableGraph<'a, V: 'a, E: 'a>: Graph<V: 'a, E: 'a> {
//
// }

pub trait Graph<'a, V: 'a, E: 'a> {
    fn get_neighbours(&'a self, v: &'a V)
        -> impl Iterator<Item=&'a V>;

    fn get_arc(&self, v: &V, u: &V) -> Option<E>;

    fn vertices_iter(&'a self) -> impl Iterator<Item=&'a V>;
}
