pub trait ExtendableGraph<'a, V: 'a, E: 'a>: Graph<'a, V, E> {
    fn push_edge(&'a mut self,
                 v: &'a V, u: &'a V, e: E); 
}

pub trait Graph<'a, V: 'a, E: 'a> {
    fn get_neighbours(&'a self, v: &'a V)
        -> impl Iterator<Item=&'a V>;
    fn get_arc(&'a self, v: &'a V, u: &'a V) -> Option<&'a E>;
    fn vertices_iter(&'a self) -> impl Iterator<Item=&'a V>;

    fn get_first(&'a self) -> &'a V {
        self.vertices_iter().next().unwrap()
    }
}
